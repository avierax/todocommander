mod model;
mod config;

use std::env;
use std::process::exit;
use config::ErrorType;
use model::*;
use directories::*;

#[derive(Debug)]
struct Error {
    message: String
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error:std::io::Error) -> Self {
        return Error {message: format!("{}", error)}
    }
}

impl std::convert::From<&str> for Error {
    fn from(error:&str) -> Self {
        return Error {message: format!("{}", error)}
    }
}

fn run_app(todo_filename: String, done_filename: String, command:config::Command) -> Result<(), Error>{
    let todo_str = std::fs::read_to_string(todo_filename)?;
    let done_str = std::fs::read_to_string(done_filename)?;
    let mut model = Model {
        todo_data : TodoData::parse(&todo_str).expect("error parsing todo file"),
        done_data : TodoData::parse(&done_str).expect("error parsing done file"),
    };
    model.execute(command).map_err(|e| Error{message: e.to_owned() })
}

fn read_configuration() -> config::Config {
    config::Config {
        todo_filename: Option::None,
        done_filename: Option::None,
    }
}

fn main() -> Result<(), Error>{
    let config = read_configuration();
    match config::parse_arguments(&mut env::args()) {
        Result::Err(ErrorType::MissingArguments(unset_arguments)) =>  {
            for unset_argument in unset_arguments {
                eprintln!("error unset argument {}", unset_argument.long_form);
            }
            Result::Err(Error{message: "missing arguments".into() })
        },
        Result::Err(ErrorType::CannotIdentifyCommand(_)) => {
            eprintln!("error cannot identify command");
            exit(2);
        }
        #[allow(unused_variables)]
        Result::Ok(arguments) => {
            // println!("Config is {:#?}", config);
            let param_todo_filename = arguments.config.todo_filename;
            let param_done_filename = arguments.config.done_filename;

            let todo_filename = param_todo_filename.or(config.todo_filename).expect("todo filename is not present in config file and --todo-file parameter is missing");
            let done_filename = param_done_filename.or(config.done_filename).expect("done filename is not present in config file and --done-file parameter is missing");
            run_app(todo_filename, done_filename, arguments.command)
        }
    }
}
