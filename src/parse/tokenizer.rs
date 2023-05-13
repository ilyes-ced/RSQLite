use std::{fmt, iter::Peekable, str::Chars};
#[derive(Debug, PartialEq)]

pub enum KeyWord {
    Select,
    Insert,
    Update,
    Delete,
    Create,
    Describe,
    Table,
    Database,
    From,
    Into,
    Values,
    Default,
    And,
    Or,
    Not,
    Null,
    Integer,
    Float,
    Text,
    Boolean,
    Drop,
    Where,
    Set,
    Distinct,
    All,
    Avg,
    Sum,
    Max,
    Min,
    NotNull,
    AutoIncrement,
    NotAKeyword,
}
#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Word(Word),
    Char(char),
    SingleQuotedString(String),
    Comma,
    Whitespace(Whitespace),
    DoubleEq,
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    LParen,
    RParen,
    Period,
    SemiColon,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
}

#[derive(Debug, PartialEq)]

pub struct Word {
    pub value: String,
    pub keyword: KeyWord,
}

#[derive(Debug)]
pub struct TokenizerError {
    pub message: String,
    pub line: u64,
    pub col: u64,
}

#[derive(Debug, PartialEq)]
pub enum Whitespace {
    Space,
    Newline,
    Tab,
    //SingleLineComment { comment: String, prefix: String },
    //MultiLineComment(String),
}

#[derive(Debug)]
pub struct Tokenizer<'a> {
    query: &'a str,
    line: u64,
    col: u64,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::EOF => f.write_str("EOF"),
            Token::Word(ref w) => write!(f, "{:?}", w),
            //Token::Number(ref n, l) => write!(f, "{}{long}", n, long = if *l { "L" } else { "" }),
            Token::Char(ref c) => write!(f, "{}", c),
            Token::SingleQuotedString(ref s) => write!(f, "'{}'", s),
            Token::Comma => f.write_str(","),
            Token::Whitespace(ws) => write!(f, "{}", ws),
            Token::DoubleEq => f.write_str("=="),
            Token::Eq => f.write_str("="),
            Token::Neq => f.write_str("<>"),
            Token::Lt => f.write_str("<"),
            Token::Gt => f.write_str(">"),
            Token::LtEq => f.write_str("<="),
            Token::GtEq => f.write_str(">="),
            Token::Plus => f.write_str("+"),
            Token::Minus => f.write_str("-"),
            Token::Mul => f.write_str("*"),
            Token::Div => f.write_str("/"),
            Token::Mod => f.write_str("%"),
            Token::LParen => f.write_str("("),
            Token::RParen => f.write_str(")"),
            Token::Period => f.write_str("."),
            Token::SemiColon => f.write_str(";"),
            Token::LBracket => f.write_str("["),
            Token::RBracket => f.write_str("]"),
            Token::LBrace => f.write_str("{"),
            Token::RBrace => f.write_str("}"),
        }
    }
}

impl fmt::Display for Whitespace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Whitespace::Space => f.write_str(" "),
            Whitespace::Newline => f.write_str("\n"),
            Whitespace::Tab => f.write_str("\t"),
            //Whitespace::SingleLineComment { prefix, comment } => write!(f, "{}{}", prefix, comment),
            //Whitespace::MultiLineComment(s) => write!(f, "/*{}*/", s),
        }
    }
}

impl fmt::Display for Word {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.value)
    }
}

impl<'a> Tokenizer<'a> {
    pub fn new(query: &'a str) -> Self {
        Self {
            query,
            line: 1,
            col: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, TokenizerError> {
        let mut peekable = self.query.chars().peekable();
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = self.next_token(&mut peekable)? {
            match &token {
                Token::Whitespace(Whitespace::Newline) => {
                    self.line += 1;
                    self.col = 1;
                }

                Token::Whitespace(Whitespace::Space) => self.col += 1,
                Token::Whitespace(Whitespace::Tab) => self.col += 4,
                Token::Word(w) => self.col += w.value.len() as u64,
                Token::SingleQuotedString(s) => self.col += s.len() as u64,
                _ => self.col += 1,
            }
            tokens.push(token);
        }
        tokens.push(Token::EOF);

        Ok(tokens)
    }

    fn next_token(&self, chars: &mut Peekable<Chars<'_>>) -> Result<Option<Token>, TokenizerError> {
        match chars.peek() {
            Some(char) => {
                match char {
                    ' ' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Space)),
                    '\t' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Tab)),
                    '\n' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Newline)),
                    '\r' => self.consume_and_return(chars, Token::Whitespace(Whitespace::Newline)),
                    '\'' => self.tokenize_single_quote_string(chars),
                    ',' => self.consume_and_return(chars, Token::Comma),
                    '=' => {
                        //self.consume_and_return(chars, Token::Eq)
                        chars.next();
                        match chars.peek() {
                            Some('=') => self.consume_and_return(chars, Token::DoubleEq),
                            _ => self.consume_and_return(chars, Token::Eq),
                        }
                    }
                    '!' => {
                        //self.consume_and_return(chars, Token::Eq)
                        chars.next();
                        match chars.peek() {
                            Some('=') => self.consume_and_return(chars, Token::Neq),
                            _ => self.consume_and_return(chars, Token::Char('!')),
                        }
                    }
                    '<' => {
                        chars.next();
                        match chars.peek() {
                            Some('=') => self.consume_and_return(chars, Token::LtEq),
                            _ => self.consume_and_return(chars, Token::Lt),
                        }
                    }
                    '>' => {
                        chars.next();
                        match chars.peek() {
                            Some('=') => self.consume_and_return(chars, Token::GtEq),
                            _ => self.consume_and_return(chars, Token::Gt),
                        }
                    }
                    '+' => self.consume_and_return(chars, Token::Plus),
                    '-' => self.consume_and_return(chars, Token::Minus),
                    '*' => self.consume_and_return(chars, Token::Mul),
                    '/' => self.consume_and_return(chars, Token::Div),
                    '%' => self.consume_and_return(chars, Token::Mod),
                    '(' => self.consume_and_return(chars, Token::LParen),
                    ')' => self.consume_and_return(chars, Token::RParen),
                    '.' => self.consume_and_return(chars, Token::Period),
                    ';' => self.consume_and_return(chars, Token::SemiColon),
                    '[' => self.consume_and_return(chars, Token::LBracket),
                    ']' => self.consume_and_return(chars, Token::RBracket),
                    '{' => self.consume_and_return(chars, Token::LBrace),
                    '}' => self.consume_and_return(chars, Token::RBrace),
                    _ => {
                        let word = self.tokenize_word(chars);
                        Ok(Some(Token::Word(word)))
                    }
                }
            }
            None => Ok(None),
        }
    }

    fn consume_and_return(
        &self,
        chars: &mut Peekable<Chars<'_>>,
        t: Token,
    ) -> Result<Option<Token>, TokenizerError> {
        chars.next();
        Ok(Some(t))
    }

    fn tokenize_single_quote_string(
        &self,
        chars: &mut Peekable<Chars<'_>>,
    ) -> Result<Option<Token>, TokenizerError> {
        // keep iterating until you find '
        chars.next();
        let mut text = String::from("");

        while let Some(&char) = chars.peek() {
            if char == '\'' {
                break;
            } else {
                text.push(char);
                chars.next();
            }
        }
        let text = String::from("hello there");
        Ok(Some(Token::SingleQuotedString(text)))
    }

    fn tokenize_word(&self, chars: &mut Peekable<Chars<'_>>) -> Word {
        let mut s = String::new();

        while let Some(char) = chars.peek() {
            if char == &' '
                || char == &'\n'
                || char == &'\r'
                || char == &'\t'
                || char == &','
                || char == &'('
                || char == &';'
            {
                break;
            } else {
                s.push(*char);
                chars.next();
            }
        }

        let word: Word = match s.as_str() {
            "select" => Word {
                value: s,
                keyword: KeyWord::Select,
            },
            "insert" => Word {
                value: s,
                keyword: KeyWord::Insert,
            },
            "update" => Word {
                value: s,
                keyword: KeyWord::Update,
            },
            "delete" => Word {
                value: s,
                keyword: KeyWord::Delete,
            },
            "create" => Word {
                value: s,
                keyword: KeyWord::Create,
            },
            "describe" => Word {
                value: s,
                keyword: KeyWord::Describe,
            },
            "table" => Word {
                value: s,
                keyword: KeyWord::Table,
            },
            "database" => Word {
                value: s,
                keyword: KeyWord::Database,
            },
            "from" => Word {
                value: s,
                keyword: KeyWord::From,
            },
            "into" => Word {
                value: s,
                keyword: KeyWord::Into,
            },
            "values" => Word {
                value: s,
                keyword: KeyWord::Values,
            },
            "default" => Word {
                value: s,
                keyword: KeyWord::Default,
            },
            "and" => Word {
                value: s,
                keyword: KeyWord::And,
            },
            "or" => Word {
                value: s,
                keyword: KeyWord::Or,
            },
            "not" => Word {
                value: s,
                keyword: KeyWord::Not,
            },
            "null" => Word {
                value: s,
                keyword: KeyWord::Null,
            },
            "integer" => Word {
                value: s,
                keyword: KeyWord::Integer,
            },
            "float" => Word {
                value: s,
                keyword: KeyWord::Float,
            },
            "text" => Word {
                value: s,
                keyword: KeyWord::Text,
            },
            "boolean" => Word {
                value: s,
                keyword: KeyWord::Boolean,
            },
            "drop" => Word {
                value: s,
                keyword: KeyWord::Drop,
            },
            "where" => Word {
                value: s,
                keyword: KeyWord::Where,
            },
            "set" => Word {
                value: s,
                keyword: KeyWord::Set,
            },
            "distinct" => Word {
                value: s,
                keyword: KeyWord::Distinct,
            },
            "all" => Word {
                value: s,
                keyword: KeyWord::All,
            },
            "avg" => Word {
                value: s,
                keyword: KeyWord::Avg,
            },
            "sum" => Word {
                value: s,
                keyword: KeyWord::Sum,
            },
            "max" => Word {
                value: s,
                keyword: KeyWord::Max,
            },
            "min" => Word {
                value: s,
                keyword: KeyWord::Min,
            },
            "not_null" => Word {
                value: s,
                keyword: KeyWord::NotNull,
            },
            "auto_increment" => Word {
                value: s,
                keyword: KeyWord::AutoIncrement,
            },
            _ => Word {
                value: s,
                keyword: KeyWord::NotAKeyword,
            },
        };

        word
    }
}