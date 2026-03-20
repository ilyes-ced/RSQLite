pub mod parser;
pub mod tokenizer;

use parser::{Allocation, Clause, Statement};

use crate::database::{table::Table, Database};

use self::parser::ColumnDef;

pub fn parse(command: String, database: &mut Database) -> Result<String, String> {
    // this block is returned
    //parser::Parser::new(command);
    match parser::Parser::parse(command) {
        Ok(result) => {
            // validate query
            for statement in result {
                let gg = match statement {
                    Statement::Select {
                        table_name,
                        all,
                        columns,
                        clauses,
                    } => validate_select((table_name, all, columns, clauses), database),
                    Statement::Insert {
                        table_name,
                        all,
                        columns,
                        values,
                    } => validate_insert((table_name, all, columns, values), database),
                    Statement::Update {
                        table_name,
                        allocations,
                        clauses,
                    } => validate_update((table_name, allocations, clauses), database),
                    Statement::Delete {
                        table_name,
                        selection,
                    } => validate_delete((table_name, selection), database),
                    Statement::CreateTable { name, columns } => {
                        validate_create((name, columns), database)
                    }
                    Statement::Drop { object_type, names } => {
                        validate_drop((object_type, names), database)
                    }
                };
                return match gg {
                    Ok(ff) => Ok(ff),
                    Err(err) => Err(err),
                };
            }
        }
        Err(err) => {
            return Err(format!(
                "message {},\ntoken: {},\ntoken index: {}",
                err.message, err.token, err.index
            ))
        }
    };

    Ok("result".to_string())
}

fn validate_create(
    params: (String, Vec<ColumnDef>),
    database: &mut Database,
) -> Result<String, String> {
    match database.has_table(&params.0) {
        true => Err(String::from("table already exists")),
        false => {
            let table_name = params.0.to_string();
            let table = Table::new(params, database)?;
            database.pager.add_table(table.1);
            table.0.show_table_structure();
            database.add_table(table_name, table.0).unwrap();
            Ok(String::from("table created"))
        }
    }
}

// check table exist
// check if selected cols exist
// check conditions cols correct and correct types
fn validate_select(
    params: (String, bool, Option<Vec<String>>, Option<Clause>),
    database: &mut Database,
) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => {
            // if all
            if params.1 {
            } else {
            }

            Ok(String::from("dazdazd"))
        }
        false => return Err(String::from("table doesnt exist")),
    }
}

// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_insert(
    params: (String, bool, Option<Vec<String>>, Vec<String>),
    database: &mut Database,
) -> Result<String, String> {
    match database.has_table(&params.0) {
        false => Err(String::from("table doesnt exists")),
        true => {
            let table = database.tables.get(&params.0).unwrap();

            println!("{:?}", table);
            println!("{:?}", params);

            //                                                           _   _       _           _     _                        _____    ___    ____     ___
            //    __ _   _   _    ___   _ __   _   _    __   __   __ _  | | (_)   __| |   __ _  | |_  (_)   ___    _ __        |_   _|  / _ \  |  _ \   / _ \
            //   / _` | | | | |  / _ \ | '__| | | | |   \ \ / /  / _` | | | | |  / _` |  / _` | | __| | |  / _ \  | '_ \         | |   | | | | | | | | | | | |
            //  | (_| | | |_| | |  __/ | |    | |_| |    \ V /  | (_| | | | | | | (_| | | (_| | | |_  | | | (_) | | | | |        | |   | |_| | | |_| | | |_| |
            //   \__, |  \__,_|  \___| |_|     \__, |     \_/    \__,_| |_| |_|  \__,_|  \__,_|  \__| |_|  \___/  |_| |_|        |_|    \___/  |____/   \___/
            //      |_|                        |___/

            let mut values: Vec<String> = Vec::new();

            //match params.2 {
            //    Some(cols) => {
            //        // check all cols exist
            //        for table_col in &table.columns {
            //            for col in &cols {
            //                if col != &table_col.name {
            //                    return Err(String::from(format!("column {} doesnt exist", col)))
            //                }
            //            }
            //        }
            //
            //    },
            //    None => {
            //
            //    }
            //}

            database.insert_row(params.0, params.2.unwrap(), params.3);

            // add data to pages
            //let cur_page = database.pager.pages[2];
            Ok(String::from("dazdazd"))
        }
    }
}

// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_update(
    params: (String, Vec<Allocation>, Clause),
    database: &mut Database,
) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist")),
    }
}

// check table exist
// validate selected cols if exist
// validate values and thier types
fn validate_delete(params: (String, Clause), database: &mut Database) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => Ok(String::from("dazdazd")),
        false => return Err(String::from("table doesnt exist")),
    }
}

fn validate_drop(params: (String, Vec<String>), database: &mut Database) -> Result<String, String> {
    match check_table_exist("table_name".to_string()) {
        true => {
            //let table = Table::new();
            //table.show_table_structure();
            // create table
            // show table structure
            // add table to database
            Ok(String::from("dazdazd"))
        }
        false => return Err(String::from("table doesnt exist")),
    }
}

fn check_table_exist(table_name: String) -> bool {
    false
}
