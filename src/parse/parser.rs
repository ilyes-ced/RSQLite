use crate::info;

use super::tokenizer::{self, KeyWord, Token, Whitespace, Word};

#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub not_null: bool,
    pub unique: bool,
    pub default: String, //pub options: Option<Vec<ColumnOptions>>,
}

#[derive(Debug)]
pub struct Allocation {
    column: String,
    value: String,
}

#[derive(Debug)]
pub enum DataType {
    Text,
    Integer,
    Float,
    Boolean,
    Null,
}

#[derive(Debug)]
pub enum Clause {
    Binop {
        left: Box<Clause>,
        operation: Token,
        right: Box<Clause>,
    },
    Value(String),
    // box needed to avoid infinite nesting
    Nested(Box<Clause>),
}

#[derive(Debug)]
pub enum Statement {
    Select {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        clauses: Option<Clause>,
    },
    Insert {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        values: Vec<String>,
    },
    Update {
        table_name: String,
        allocations: Vec<Allocation>,
        clauses: Clause,
    },
    Delete {
        table_name: String,
        selection: Clause,
    },
    CreateTable {
        name: String,
        columns: Vec<ColumnDef>,
    },
    Drop {
        object_type: String,
        names: Vec<String>,
    },
}
#[derive(Debug)]
pub struct ParserError {
    pub message: String,
    pub token: Token,
    pub index: String,
}

#[derive(Debug)]
pub struct Parser /*<'a>*/ {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser /*<'a>*/ {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, index: 0 }
    }

    pub fn parse(query: String) -> Result<Vec<Statement>, ParserError> {
        let mut new_tokenizer = tokenizer::Tokenizer::new(&query);
        // might need error handling
        let tokens = match new_tokenizer.tokenize() {
            Ok(value) => value,
            Err(err) => {
                return Err(ParserError {
                    message: err.message,
                    // random for testing idk if needed
                    token: Token::Comma,
                    index: String::from(format!("{}", err.col * err.line)),
                });
            }
        };

        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();

        loop {
            if parser.check_file_end() {
                break;
            }

            let statement = parser.parse_statement()??;
            statements.push(statement);

            parser.index += 1;
        }

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<Result<Statement, ParserError>, ParserError> {
        // this block is returned
        let gg = match &self.tokens[self.index] {
            Token::Word(key_word) => match key_word.keyword {
                KeyWord::Select => Ok(self.select_statement()),
                KeyWord::Insert => Ok(self.insert_statement()),
                KeyWord::Update => Ok(self.update_statement()),
                KeyWord::Delete => Ok(self.delete_statement()),
                KeyWord::Create => Ok(self.create_statement()),
                KeyWord::Drop => Ok(self.drop_statement()),
                _ => return Err(self.return_error("no keywords")),
            },
            _ => return Err(self.return_error("idk the error")),
        };
        gg
    }

    pub fn select_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        match self.tokens[self.index] {
            Token::Mul => {
                all = true;
                self.next_token();
            }
            _ => match self.tokens[self.index] {
                Token::LParen => {
                    cols = self.get_words_in_paren()?;
                }
                _ => return Err(self.return_error("columns are req")),
            },
        }

        self.confirm_keyword(KeyWord::From)?;

        let table_name = self.get_object_name()??;

        let clauses = self.get_clauses()?;

        // makes sure notthing is after last token
        self.finalize_query()?;

        Ok(Statement::Select {
            table_name,
            all,
            columns: if cols.len() != 0 { Some(cols) } else { None },
            clauses: if clauses.is_some() { clauses } else { None },
        })
    }

    pub fn insert_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        self.confirm_keyword(KeyWord::Into)?;

        let table_name = self.get_object_name()??;

        match &mut self.tokens[self.index] {
            Token::LParen => {
                // get table cols insert into table_name(col1, col2, col3)
                cols = self.get_words_in_paren()?;
            }
            Token::Word(Word {
                keyword: KeyWord::Values,
                ..
            }) => {
                // if next token is Token::Values means the columns are unspecified so all of them
                all = true;
                self.next_token();
                if self.tokens[self.index] != Token::LParen {
                    return Err(self.return_error("values not found"));
                }
                values = self.get_values_in_paren()?;
            }
            _ => return Err(self.return_error("invalid syntax after table name")),
        };

        if all == false {
            match &mut self.tokens[self.index] {
                Token::Word(Word {
                    keyword: KeyWord::Values,
                    ..
                }) => {
                    self.next_token();
                    values = self.get_values_in_paren()?;
                }
                _ => return Err(self.return_error("invalid syntax after table name")),
            };
        }

        // makes sure notthing is after last token
        self.finalize_query()?;

        Ok(Statement::Insert {
            table_name,
            all,
            columns: if cols.len() != 0 { Some(cols) } else { None },
            values,
        })
    }

    pub fn update_statement(&mut self) -> Result<Statement, ParserError> {
        let mut allocations: Vec<Allocation> = Vec::new();
        self.next_token();

        let table_name = self.get_object_name()??;

        match self.tokens[self.index] {
            Token::Word(Word {
                keyword: KeyWord::Set,
                ..
            }) => {
                self.next_token();
                loop {
                    let mut allocation = Allocation {
                        column: String::new(),
                        value: String::new(),
                    };
                    match &self.tokens[self.index] {
                        Token::Word(Word { value: col, .. }) => {
                            allocation.column = col.to_string();
                            self.next_token();
                        }
                        _ => return Err(self.return_error("column name wher error")),
                    }
                    match &self.tokens[self.index] {
                        Token::Eq => {
                            self.next_token();
                        }
                        _ => return Err(self.return_error("no = sign")),
                    }
                    match &self.tokens[self.index] {
                        Token::SingleQuotedString(value) => {
                            allocation.value = value.to_string();
                            self.next_token();
                        }
                        _ => return Err(self.return_error("string values must be qouted")),
                    }
                    allocations.push(allocation);
                    match &self.tokens[self.index] {
                        Token::Word(Word {
                            keyword: KeyWord::And,
                            ..
                        }) => {
                            self.next_token();
                        }
                        _ => {
                            break;
                        }
                    }
                }
            }
            _ => {}
        }

        let clauses = self.get_clauses()?;
        // makes sure notthing is after last token
        self.finalize_query()?;
        if clauses.is_some() {
            return Ok(Statement::Update {
                table_name,
                clauses: clauses.unwrap(),
                allocations,
            });
        } else {
            return Err(self.return_error("conditions are required"));
        }
    }

    pub fn delete_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();

        self.confirm_keyword(KeyWord::From)?;
        let table_name = self.get_object_name()??;
        let clauses = self.get_clauses()?;
        self.finalize_query()?;

        if clauses.is_some() {
            return Ok(Statement::Delete {
                table_name,
                selection: clauses.unwrap(),
            });
        } else {
            return Err(self.return_error("conditions are required"));
        }
    }

    pub fn drop_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();
        let mut drop: Vec<String> = Vec::new();

        loop {
            match &mut self.tokens[self.index] {
                Token::SemiColon => break,
                Token::Word(Word { value, .. }) => {
                    drop.push(value.to_string());
                    self.next_token();
                }
                _ => return Err(self.return_error("invalid syntax after table name")),
            };
        }
        if drop.len() == 0 {
            return Err(self.return_error("nothing selected"));
        }
        self.finalize_query()?;

        Ok(Statement::Drop {
            object_type: String::from("table"),
            names: drop,
        })
    }

    pub fn create_statement(&mut self) -> Result<Statement, ParserError> {
        self.next_token();
        self.confirm_keyword(KeyWord::Table)?;
        // table name
        let name = self.get_object_name()??;
        self.next_token();

        let mut columns: Vec<ColumnDef> = Vec::new();

        loop {
            let mut column = ColumnDef {
                name: name.to_string(),
                data_type: DataType::Null,
                primary_key: false,
                auto_increment: false,
                not_null: false,
                unique: false,
                default: String::new(),
            };
            // get col name
            match &self.tokens[self.index] {
                Token::Word(Word {
                    value: col_name, ..
                }) => column.name = col_name.to_string(),
                _ => return Err(self.return_error("col name is required")),
            }
            self.next_token();
            // get datatype
            match &self.tokens[self.index] {
                Token::Word(Word {
                    keyword: KeyWord::Integer,
                    ..
                }) => column.data_type = DataType::Integer,
                Token::Word(Word {
                    keyword: KeyWord::Float,
                    ..
                }) => column.data_type = DataType::Float,
                Token::Word(Word {
                    keyword: KeyWord::Boolean,
                    ..
                }) => column.data_type = DataType::Boolean,
                Token::Word(Word {
                    keyword: KeyWord::Text,
                    ..
                }) => column.data_type = DataType::Text,
                //Token::Word(Word { keyword: KeyWord::Null,    .. }) => column.data_type = DataType::Null,
                _ => return Err(self.return_error("col type req [integer, float, boolean, text]")),
            }
            self.next_token();
            // get column paramaters [notnull unique primaryKey default()]
            loop {
                match &self.tokens[self.index] {
                    Token::Comma => {
                        self.next_token();
                        columns.push(column);
                        break;
                    }
                    Token::Word(Word {
                        keyword: KeyWord::PrimaryKey,
                        ..
                    }) => column.primary_key = true,
                    Token::Word(Word {
                        keyword: KeyWord::NotNull,
                        ..
                    }) => column.not_null = true,
                    Token::Word(Word {
                        keyword: KeyWord::AutoIncrement,
                        ..
                    }) => column.auto_increment = true,
                    Token::Word(Word {
                        keyword: KeyWord::Unique,
                        ..
                    }) => column.unique = true,
                    Token::Word(Word {
                        keyword: KeyWord::Default,
                        ..
                    }) => {
                        self.next_token();
                        match &self.tokens[self.index] {
                            Token::LParen => {}
                            _ => return Err(self.return_error("no default value")),
                        }
                        self.next_token();
                        match column.data_type {
                            DataType::Integer => match &self.tokens[self.index] {
                                Token::Number(s, b) => column.default = "true".to_string(),
                                _ => {
                                    return Err(self.return_error(
                                        "wrong datatype for default: expected integer",
                                    ))
                                }
                            },
                            DataType::Float => match &self.tokens[self.index] {
                                Token::Number(s, b) => column.default = s.to_string(),
                                _ => {
                                    return Err(self.return_error(
                                        "wrong datatype for default: expected float",
                                    ))
                                }
                            },
                            DataType::Boolean => match &self.tokens[self.index] {
                                Token::Word(Word {
                                    keyword: KeyWord::True,
                                    ..
                                }) => column.default = "true".to_string(),
                                Token::Word(Word {
                                    keyword: KeyWord::False,
                                    ..
                                }) => column.default = "false".to_string(),
                                _ => {
                                    return Err(self.return_error(
                                        "wrong datatype for default: expected true or false",
                                    ))
                                }
                            },
                            DataType::Text => match &self.tokens[self.index] {
                                Token::SingleQuotedString(value) => {
                                    column.default = value.to_string()
                                }
                                _ => {
                                    return Err(self.return_error(
                                        "wrong datatype for default: expected 'string'",
                                    ))
                                }
                            },
                            // might remove
                            DataType::Null => match &self.tokens[self.index] {
                                Token::Number(s, b) => column.default = s.to_string(),
                                _ => return Err(self.return_error("wrong datatype for default")),
                            },
                        };
                        self.next_token();
                        match &self.tokens[self.index] {
                            Token::RParen => {}
                            _ => return Err(self.return_error("default value not closed")),
                        }
                    }
                    _ => {
                        return Err(self.return_error(&format!(
                            "invalid column option {}",
                            &self.tokens[self.index]
                        )))
                    }
                }
                //self.next_token();
                self.next_token();
                if self.tokens[self.index] == Token::Comma {
                    self.next_token();
                    columns.push(column);
                    break;
                }
                if self.tokens[self.index] == Token::RParen {
                    columns.push(column);
                    break;
                }
            }
            if self.tokens[self.index] == Token::RParen {
                self.next_token();
                break;
            }
        }

        self.finalize_query()?;

        Ok(Statement::CreateTable { name, columns })
    }

    // object is table_name col
    pub fn get_object_name(&mut self) -> Result<Result<String, ParserError>, ParserError> {
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index] {
            Token::Word(Word {
                value: table_name, ..
            }) => Ok(table_name.to_string()),
            _ => return Err(self.return_error("idk table name error")),
        };
        self.next_token();
        Ok(table_name)
    }

    pub fn finalize_query(&self) -> Result<(), ParserError> {
        match self.tokens[self.index] {
            Token::SemiColon => return Ok(()),
            _ => {
                return Err(self.return_error(&format!(
                    "invalid syntax at the end: {:?}",
                    self.tokens[self.index]
                )))
            }
        };
    }

    pub fn get_words_in_paren(&mut self) -> Result<Vec<String>, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        self.next_token();
        loop {
            match &self.tokens[self.index] {
                Token::Word(Word { value: col, .. }) => {
                    cols.push(col.to_string());
                    self.next_token();
                }
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                }
                _ => return Err(self.return_error("cols req")),
            }
        }
        Ok(cols)
    }

    pub fn get_values_in_paren(&mut self) -> Result<Vec<String>, ParserError> {
        let mut values: Vec<String> = Vec::new();
        self.next_token();
        loop {
            match &self.tokens[self.index] {
                Token::SingleQuotedString(value) => {
                    values.push(value.to_string());
                    self.next_token();
                }
                Token::Number(value, sign) => {
                    values.push(value.to_string());
                    self.next_token();
                }
                Token::Comma => self.next_token(),
                Token::RParen => {
                    self.next_token();
                    break;
                }
                _ => return Err(self.return_error("values req")),
            }
        }
        Ok(values)
    }

    pub fn next_token(&mut self) {
        self.index += 1;
        loop {
            match &self.tokens[self.index] {
                //Token::Comma => self.index += 1,
                Token::Whitespace(Whitespace::Space) => self.index += 1,
                Token::Whitespace(Whitespace::Newline) => self.index += 1,
                Token::Whitespace(Whitespace::Tab) => self.index += 1,
                //Token::Word(Word { keyword: KeyWord::Select, .. }) => self.index += 1,
                _ => break,
            }
        }
    }

    pub fn confirm_keyword(&mut self, expected_keyword: KeyWord) -> Result<(), ParserError> {
        if let Token::Word(Word { keyword, .. }) = &self.tokens[self.index] {
            if keyword == &expected_keyword {
                self.next_token();
                return Ok(());
            } else {
                return Err(self.return_error("unexpected keyword"));
            }
        }
        return Err(self.return_error("no keyword"));
    }

    pub fn check_file_end(&mut self) -> bool {
        let result = match self.tokens[self.index] {
            Token::EOF => true,
            _ => false,
        };
        result
    }

    //TODO: needs more formatting
    pub fn get_clauses(&mut self) -> Result<Option<Clause>, ParserError> {
        match self.tokens[self.index] {
            Token::Word(Word {
                keyword: KeyWord::Where,
                ..
            }) => {}
            _ => return Ok(None),
        };

        self.next_token();
        let mut selection = match self.tokens[self.index] {
            Token::Word(Word { .. }) => self.get_binary_operator(),
            Token::LParen => self.get_nested_binary_operators(),
            _ => return Err(self.return_error("invalid syntax after where")),
        }?;
        self.next_token();

        loop {
            if self.tokens[self.index] == Token::SemiColon {
                break;
            }

            selection = match self.tokens[self.index] {
                Token::Word(Word {
                    keyword: KeyWord::And,
                    ..
                }) => {
                    self.next_token();
                    let next_clause = match self.tokens[self.index] {
                        Token::Word(Word { .. }) => self.get_binary_operator(),
                        Token::LParen => self.get_nested_binary_operators(),
                        _ => return Err(self.return_error("invalid syntax after where")),
                    }?;
                    Clause::Binop {
                        left: Box::new(selection),
                        operation: Token::Word(Word {
                            keyword: KeyWord::And,
                            value: "and".to_string(),
                        }),
                        right: Box::new(next_clause),
                    }
                }
                Token::Word(Word {
                    keyword: KeyWord::Or,
                    ..
                }) => {
                    self.next_token();
                    let next_clause = match self.tokens[self.index] {
                        Token::Word(Word { .. }) => self.get_binary_operator(),
                        Token::LParen => self.get_nested_binary_operators(),
                        _ => return Err(self.return_error("invalid syntax after where")),
                    }?;
                    Clause::Binop {
                        left: Box::new(selection),
                        operation: Token::Word(Word {
                            keyword: KeyWord::Or,
                            value: "or".to_string(),
                        }),
                        right: Box::new(next_clause),
                    }
                }
                _ => break,
            };
            self.next_token();
        }

        Ok(Some(selection))
    }

    pub fn get_binary_operator(&mut self) -> Result<Clause, ParserError> {
        let col_name = match &self.tokens[self.index] {
            Token::Word(Word { value, .. }) => value.to_string(),
            _ => {
                return Err(self.return_error(&format!(
                    "col name in where wrong {:?}",
                    self.tokens[self.index]
                )))
            }
        };
        self.next_token();

        let operator = match self.tokens[self.index] {
            Token::Eq => Token::Eq,
            Token::Gt => Token::Gt,
            Token::Lt => Token::Lt,
            Token::GtEq => Token::GtEq,
            Token::LtEq => Token::LtEq,
            Token::Neq => Token::Neq,
            _ => {
                return Err(
                    self.return_error(&format!("invalid operator {}", self.tokens[self.index]))
                )
            }
        };
        self.next_token();

        let value = match &self.tokens[self.index] {
            Token::SingleQuotedString(value) => value.to_string(),
            Token::Number(value, ..) => value.to_string(),
            Token::Word(Word {
                keyword: KeyWord::True,
                ..
            }) => "true".to_string(),
            Token::Word(Word {
                keyword: KeyWord::False,
                ..
            }) => "false".to_string(),
            _ => return Err(self.return_error("value name in where wrong")),
        };

        Ok(Clause::Binop {
            left: Box::new(Clause::Value(col_name)),
            operation: operator,
            right: Box::new(Clause::Value(value)),
        })
    }

    pub fn get_nested_binary_operators(&mut self) -> Result<Clause, ParserError> {
        match &self.tokens[self.index] {
            Token::LParen => {}
            _ => return Err(self.return_error("this error is probably impossible")),
        }
        self.next_token();

        let mut selection = match self.tokens[self.index] {
            Token::Word(Word { .. }) => self.get_binary_operator(),
            Token::LParen => self.get_nested_binary_operators(),
            _ => return Err(self.return_error("invalid syntax after where")),
        }?;
        self.next_token();

        loop {
            if self.tokens[self.index] == Token::SemiColon {
                break;
            }

            selection = match self.tokens[self.index] {
                Token::Word(Word {
                    keyword: KeyWord::And,
                    ..
                }) => {
                    self.next_token();
                    let next_clause = match self.tokens[self.index] {
                        Token::Word(Word { .. }) => self.get_binary_operator(),
                        Token::LParen => self.get_nested_binary_operators(),
                        _ => return Err(self.return_error("invalid syntax after where")),
                    }?;
                    self.next_token();
                    Clause::Binop {
                        left: Box::new(selection),
                        operation: Token::Word(Word {
                            keyword: KeyWord::And,
                            value: "and".to_string(),
                        }),
                        right: Box::new(next_clause),
                    }
                }
                Token::Word(Word {
                    keyword: KeyWord::Or,
                    ..
                }) => {
                    self.next_token();
                    let next_clause = match self.tokens[self.index] {
                        Token::Word(Word { .. }) => self.get_binary_operator(),
                        Token::LParen => self.get_nested_binary_operators(),
                        _ => return Err(self.return_error("invalid syntax after where")),
                    }?;
                    self.next_token();
                    Clause::Binop {
                        left: Box::new(selection),
                        operation: Token::Word(Word {
                            keyword: KeyWord::Or,
                            value: "or".to_string(),
                        }),
                        right: Box::new(next_clause),
                    }
                }
                _ => break,
            };
        }

        match &self.tokens[self.index] {
            Token::RParen => {}
            _ => return Err(self.return_error("right parenteses not found")),
        }
        Ok(Clause::Nested(Box::new(selection)))
    }

    pub fn return_error(&self, message: &str) -> ParserError {
        let token_errored = self.tokens.get(self.index).unwrap();
        let mut line = 1;
        let mut col = 1;

        for token in &self.tokens {
            if token == &self.tokens[self.index] {
                break;
            }
            match token {
                Token::Whitespace(Whitespace::Newline) => {
                    line += 1;
                    col += 1;
                }
                Token::Whitespace(Whitespace::Space) => col += 1,
                Token::Whitespace(Whitespace::Tab) => col += 4,
                Token::Word(w) => col += w.value.len() as u64,
                Token::SingleQuotedString(s) => col += s.len() as u64,
                Token::DoubleEq => col += 2,
                Token::Neq => col += 2,
                Token::LtEq => col += 2,
                Token::GtEq => col += 2,
                _ => col += 1,
            }
        }

        ParserError {
            message: message.to_string(),
            token: token_errored.clone(),
            index: format!("\n\tline: {},\n\tcol: {}", line, col),
        }
    }
}
