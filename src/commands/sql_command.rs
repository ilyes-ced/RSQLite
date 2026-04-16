use crate::database::Database;
use crate::parse;
use crate::parse::parser::{Parser, ParserError, Statement};
#[derive(Debug)]
pub enum SQLCommand {
    Insert,
    Select,
    Create,
    Update,
    Delete,
    Drop,
    Invalid(String),
}

impl SQLCommand {
    pub fn new(command: String) -> SQLCommand {
        let args: Vec<&str> = command.split_whitespace().collect();
        let cmd = args[0].to_owned().to_lowercase();

        match cmd.as_ref() {
            "insert" => SQLCommand::Insert,
            "select" => SQLCommand::Select,
            "create" => SQLCommand::Create,
            "update" => SQLCommand::Update,
            "delete" => SQLCommand::Delete,
            "drop" => SQLCommand::Drop,
            _ => SQLCommand::Invalid(format!("invalid query type: {}", cmd)),
        }
    }
}

pub fn run_sql_command(command: String, database: &mut Database) -> Result<String, String> {
    match parse::parse(command.trim().to_string(), database) {
        Ok(msg) => return Ok(format!("{}", msg)),
        Err(msg) => return Err(format!("{}", msg)),
    };

    // here we call the database and table on parsing success
}

//match Parser::parse(command.trim().to_string()) {
//    Ok(msg) => return Ok(format!("{:?}", msg)),
//    Err(msg) => return Err(format!("{:?}", msg)),
//;
