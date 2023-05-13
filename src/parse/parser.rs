use super::tokenizer::{self, Token, KeyWord, Word, Whitespace};

#[derive(Debug)]
pub struct ColumnDef {
    pub name: String,
    pub data_type: DataType,
    pub options: Vec<ColumnOptionDef>,
}
#[derive(Debug)]

pub struct Selection {
    column: ColumnDef,
    value: DataType, // changing type
}
#[derive(Debug)]


pub struct Clause {
    column: String,
    value: String,
}
#[derive(Debug)]

// like not_null or auto_increment
pub struct ColumnOptionDef {
    pub name: Option<String>,
    pub option: ColumnOption,
}
#[derive(Debug)]

pub enum DataType {
    Text(u32),    //lenght
    Integer(u32), //lenght
    Float(u32),   //lenght
    Null,
}
#[derive(Debug)]

pub enum ObjectType {
    Table,
}
#[derive(Debug)]

pub enum ColumnOption {
    Null,
    NotNull,
    Default(),
    Unique { is_primary: bool },
    Check(Selection),
}

#[derive(Debug)]
pub enum Statement {
    Select {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        selection: Option<Vec<Clause>>,
    },
    Insert {
        table_name: String,
        all: bool,
        columns: Option<Vec<String>>,
        values: Vec<String>,
    },
    Update {
        table: String,
        Clauses: Vec<Clause>,
        selection: Option<Clause>,
    },
    Delete {
        table_name: String,
        selection: Option<Clause>,
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
    index: usize,
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
        let tokens = new_tokenizer.tokenize().unwrap();
        println!("{:?}", tokens);
        let mut parser = Parser::new(tokens);
        let mut statements: Vec<Statement> = Vec::new();

        loop{
            // not needeed parsers take care of query end
            //if parser.check_query_end() {
            //    println!("add statement here");
            //}
            if parser.check_file_end() {
                break;
            }
            
            let statement = parser.parse_statement()??;
            statements.push(statement);
            
            parser.index += 1;
        }

        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<Result<Statement, ParserError>, ParserError>  {
        println!("parse_statement start");
        match &self.tokens[self.index]{
            Token::Word(key_word) => {
                match key_word.keyword{
                    KeyWord::Select => Ok(self.select_statement()),
                    KeyWord::Insert => Ok(self.insert_statement()),
                    KeyWord::Update => Ok(self.update_statement()),
                    KeyWord::Delete => Ok(self.delete_statement()),
                    KeyWord::Create => Ok(self.create_statement()),
                    _ => Err(ParserError{
                        message: "no keyword".to_owned(),
                        index: self.index
                    }),
                }
            }
            _ => Err(ParserError{
                message: "idk what the error".to_owned(),
                index: self.index
            }),
        }
    }











    pub fn select_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut all = false;
        let mut selection: Vec<Clause> = Vec::new();
        self.next_token();
    
        if self.tokens[self.index] == Token::Mul {
            all = true;
            self.next_token();
        }else {
            loop{
                match &self.tokens[self.index]{
                    Token::Word(Word { keyword: KeyWord::NotAKeyword, value: col }) => {
                        cols.push(col.to_string());
                        self.next_token();
                    },
                    Token::Comma => self.next_token(),
                    Token::Word(Word { keyword: KeyWord::From, .. }) => break,
                    _ => return Err(
                        ParserError {
                            message: String::from("columns are required"),
                            index: self.index,
                        }
                    )
                }
            }
        }
    
        match &self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::From, .. }) => {
                self.next_token();
            },
            _ => {
                return Err(
                    ParserError {
                        message: "no from".to_owned(),
                        index: self.index,
                    }
                )
            }
        }
    
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word {keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        // idk why this cant be put before Ok(table_name.to_string())
        self.next_token();
    


        

        // handle where here
        match self.tokens[self.index]{
            Token::Word(Word {keyword: KeyWord::Where, ..}) => {
                self.next_token();
                loop{
                    let mut clause = Clause{
                        column: String::new(),
                        value: String::new(),
                    };
                    match &self.tokens[self.index]{
                        Token::Word(Word { value: col, .. }) => {
                            clause.column = col.to_string();
                            self.next_token();
                        },
                        _ => {
                            return Err(         
                                ParserError {
                                    message: "column name wher error".to_owned(),
                                    index: self.index,         
                                }         
                            )           
                        }
                    }
                    match &self.tokens[self.index]{
                        Token::Eq => {
                            self.next_token();
                        },
                        _ => {
                            return Err(
                                ParserError {
                                    message: "no = sign".to_owned(),
                                    index: self.index,
                                }
                            )
                        }
                    }
                    match &self.tokens[self.index]{
                        Token::Word(Word { value, .. }) => {
                            clause.value = value.to_string();
                            self.next_token();
                        },
                        _ => {
                            return Err(
                                ParserError {
                                    message: "error in values".to_owned(),
                                    index: self.index,
                                }
                            )
                        }
                    }
                    selection.push(clause);
                    match &self.tokens[self.index]{
                        Token::Word(Word {keyword: KeyWord::And, .. }) => {
                            self.next_token();
                        },
                        _ => {
                            break       
                        }
                    }
                    // if AND continue else break
                }
            }
            _ => {}
        }


        // makes sure notthing is after last token
        match &mut self.tokens[self.index]{
            Token::SemiColon => {},
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    


        

        
        Ok(Statement::Select {
            table_name: table_name?,
            all: all,
            columns: if cols.len() != 0 { Some(cols) }else { None },
            selection: if selection.len() != 0 { Some(selection) }else { None },
        })
    }

























    pub fn insert_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();
    
    
        match &self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::Into, .. }) => {
                self.next_token();
            },
            _ => {
                return Err(
                    ParserError {
                        message: "no into".to_owned(),
                        index: self.index,
                    }
                )
            }
        }
    
        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        self.next_token();
    
        match &mut self.tokens[self.index]{
            Token::LParen => {
                self.next_token();
                loop{
                    println!("{:?}", &self.tokens[self.index]);
                    match &self.tokens[self.index]{
                        Token::Word(Word { value: col, .. }) => {
                            cols.push(col.to_string());
                            self.next_token();
                        },
                        Token::Comma => self.next_token(),
                        Token::RParen => {
                            self.next_token();
                            break;
                        },
                        _ => return Err(
                            ParserError {
                                message: String::from("error in columns"),
                                index: self.index,
                            }
                        )
                    }
                }
        
            },
            Token::Word(Word { keyword: KeyWord::Values, .. }) => {
                all = true;
                if self.tokens[self.index] != Token::LParen {
                    return Err(
                        ParserError {
                            message: "values not found".to_owned(),
                            index: self.index,
                        }
                    )
                }
                self.next_token();
                loop{
                    match &self.tokens[self.index]{
                        Token::SingleQuotedString(value) => {
                            values.push(value.to_string());
                            self.next_token();
                        },
                        Token::Comma => self.next_token(),
                        Token::RParen => {
                            self.next_token();
                            break;
                        },
                        _ => return Err(
                            ParserError {
                                message: String::from("values are required"),
                                index: self.index,
                            }
                        )
                    }
                }
            },
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    


        if all == false{
            match &mut self.tokens[self.index]{
                Token::Word(Word { keyword: KeyWord::Values, .. }) => {
                    self.next_token();
                    self.next_token();
                    loop{
                        match &self.tokens[self.index]{
                            Token::SingleQuotedString(value) => {
                                values.push(value.to_string());
                                self.next_token();
                            },
                            Token::Comma => self.next_token(),
                            Token::RParen => {
                                self.next_token();
                                break;
                            },
                            _ => return Err(
                                ParserError {
                                    message: String::from("values are required"),
                                    index: self.index,
                                }
                            )
                        }
                    }
                },
                _ => {
                    return Err(
                        ParserError {
                            message: "invalid syntax after table name".to_owned(),
                            index: self.index,
                        }
                    )
                }
            };
        }


        // makes sure notthing is after last token
        match &mut self.tokens[self.index]{
            Token::SemiColon => {},
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
    
    
        Ok(Statement::Insert {
            table_name: table_name?,
            all: all,
            columns: Some(cols),
            values: values,
        })
    
    }













































    pub fn update_statement(&mut self) -> Result<Statement, ParserError> {
        let mut cols: Vec<String> = Vec::new();
        let mut values: Vec<String> = Vec::new();
        let mut all = false;
        self.next_token();

        let table_name: Result<String, ParserError> = match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::NotAKeyword, value: table_name}) => {
                Ok(table_name.to_string())
            },
            _ => {
                return Err(
                    ParserError {
                        message: "idk table name error".to_owned(),
                        index: self.index,
                    }
                )
            }
        };
        self.next_token();






        match &mut self.tokens[self.index]{
            Token::Word(Word { keyword: KeyWord::And, .. }) => {
                self.next_token();
                loop{
                    println!("{:?}", &self.tokens[self.index]);
                    match &self.tokens[self.index]{
                        Token::Word(Word { value: col, .. }) => {
                            cols.push(col.to_string());
                            self.next_token();
                        },
                        Token::Comma => self.next_token(),
                        Token::RParen => {
                            self.next_token();
                            break;
                        },
                        _ => return Err(
                            ParserError {
                                message: String::from("error in columns"),
                                index: self.index,
                            }
                        )
                    }
                }
        
            },
            Token::Word(Word { keyword: KeyWord::Set, .. }) => {
                // get col name
                // get =
                // get value 
            },
            _ => {
                return Err(
                    ParserError {
                        message: "invalid syntax after table name".to_owned(),
                        index: self.index,
                    }
                )
            }
        };









        self.next_token();
        Ok(Statement::Update {
            table: table_name?,
            Clauses: todo!(),
            selection: todo!()
        })
    }












































    pub fn delete_statement(&mut self) -> Result<Statement, ParserError> {
        println!("delete_");
        self.next_token();
        Ok(Statement::Delete {
            table_name: todo!(),
            selection: todo!()
        })
    }
    pub fn create_statement(&mut self) -> Result<Statement, ParserError> {
        println!("create_");
        self.next_token();
        Ok(Statement::CreateTable {
            name: todo!(),
            columns: todo!()
        })
    }





    pub fn next_token (&mut self) {
        self.index += 1;
        loop{
            match &self.tokens[self.index]{
                Token::Comma => self.index += 1,
                Token::Whitespace(Whitespace::Space) => self.index += 1,
                Token::Whitespace(Whitespace::Newline) => self.index += 1,
                Token::Whitespace(Whitespace::Tab) => self.index += 1,
                //Token::Word(Word { keyword: KeyWord::Select, .. }) => self.index += 1,
                _ => break
            }
        }
    }


    pub fn confirm_keyword(keyword: KeyWord){

    }








    //pub fn check_query_end(&mut self) -> bool {
    //    let result = match self.tokens[self.index]{
    //        Token::SemiColon => true,
    //        _ => false
    //    };
    //    result
    //}

    pub fn check_file_end(&mut self) -> bool {
        let result = match self.tokens[self.index]{
            Token::EOF => true,
            _ => false
        };
        result
    }

}






// insert into users values('fez', 'zefzef' ,'fzefze');
