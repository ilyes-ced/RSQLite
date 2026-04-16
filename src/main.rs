use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::{env, path::Path, process};

mod commands;
mod constants;
mod database;
mod engine;
mod log;
mod parse;
mod rustyline_config;
mod utils;
use utils::init_file::file_init;

use commands::{
    meta_command::run_meta_command, process_command, sql_command::run_sql_command, CommandType,
};
use database::Database;
use rustyline_config::{get_config, REPLHelper};

use crate::commands::sql_command::SQLCommand;

fn main() -> rustyline::Result<()> {
    let args: Vec<String> = env::args().collect();
    info!("{:?}", args);
    if args.len() == 1 {
        error!("expected database file name");
        process::exit(1)
    } else if args.len() > 2 {
        error!("unexpected arguments");
        process::exit(1)
    }

    if args[0] == "--help" || args[0] == "-h" {
        info!("help message");
        process::exit(1)
    } else if args[0] == "--version" || args[0] == "-v" {
        info!("0.1.0");
        process::exit(1)
    }

    let mut database = if Path::new(&args[1]).exists() {
        // maybe match here
        let file = file_init(&args[1]).unwrap();
        Database::new(args[1].to_string(), file)
    } else {
        error!("invalid database file");
        process::exit(1)
    };

    let config = get_config();
    let helper = REPLHelper::default();
    let mut repl = Editor::with_config(config).unwrap();
    repl.set_helper(Some(helper));

    if repl.load_history("history.txt").is_err() {
        warning!("No previous history.");
    }

    let prompt = format!("RSQL> ");
    repl.helper_mut().expect("No helper found").colored_prompt =
        format!("\x1b[1;34m{}\x1b[0m", prompt);

    loop {
        let readline = repl.readline(&prompt);
        match readline {
            Ok(command) => {
                if !command.is_empty() {
                    let _ = repl.add_history_entry(command.as_str());
                    match process_command(&command) {
                        CommandType::TypeSQL(cmd) => match cmd {
                            SQLCommand::Invalid(err) => error!("{}", err),
                            _ => match run_sql_command(command, &mut database) {
                                Ok(result) => info!("{}", result),
                                Err(err) => error!("{}", err),
                            },
                        },
                        CommandType::TypeMeta(cmd) => match run_meta_command(cmd) {
                            Ok(result) => info!("{}", result),
                            Err(err) => error!("{}", err),
                        },
                    };
                }
            }
            Err(ReadlineError::Interrupted) => {
                info!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                info!("CTRL-D");
                break;
            }
            Err(err) => {
                error!("Error: {:?}", err);
                break;
            }
        }
    }

    let _ = repl.save_history("history.txt");

    Ok(())
}
