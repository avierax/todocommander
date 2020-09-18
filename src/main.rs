mod config;
mod model;

use config::ErrorType;
use model::*;
use std::env;
use std::io::prelude::*;
use std::process::exit;

#[derive(Debug)]
struct Error {
    message: String,
}

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        return Error {
            message: format!("{}", error),
        };
    }
}

impl std::convert::From<&str> for Error {
    fn from(error: &str) -> Self {
        return Error {
            message: format!("{}", error),
        };
    }
}

struct Config {
    todo_filename: Option<String>,
    done_filename: Option<String>,
}

fn run_app(
    todo_filename: String,
    done_filename: String,
    command: config::Command,
) -> Result<(), Error> {
    let todo_str = std::fs::read_to_string(todo_filename)?;
    let done_str = std::fs::read_to_string(done_filename)?;
    let mut model = Model {
        todo_data: TodoData::parse(&todo_str).expect("error parsing todo file"),
        done_data: TodoData::parse(&done_str).expect("error parsing done file"),
    };
    model.execute(command).map_err(|e| Error {
        message: e.to_owned(),
    })
}

fn read_configuration() -> Config {
    let mut result = Config {
        todo_filename: Option::None,
        done_filename: Option::None,
    };
    if let Option::Some(dirs) = directories::ProjectDirs::from("", "", "todocommander") {
        let mut path = dirs.config_dir().to_path_buf();
        path.push("todocommander.cfg");
        if let Result::Ok(mut f) = std::fs::File::open(path) {
            let mut file_content = String::new();
            let _result = f.read_to_string(&mut file_content);
            file_content.lines().for_each(|l| {
                let line: &str = l;
                let split: Vec<&str> = line.split('=').collect();
                if split[0] == "todo_filename" {
                    result.todo_filename = Option::Some(split[1].to_owned());
                }
                if split[1] == "done_filename" {
                    result.done_filename = Option::Some(split[1].to_owned());
                }
            })
        }
    }
    result
}

fn main() -> Result<(), Error> {
    let config = read_configuration();
    match config::parse_arguments(&mut env::args()) {
        Result::Err(ErrorType::MissingArguments(unset_arguments)) => {
            for unset_argument in unset_arguments {
                eprintln!("error unset argument {}", unset_argument.long_form);
            }
            Result::Err(Error {
                message: "missing arguments".into(),
            })
        }
        Result::Err(ErrorType::CannotIdentifyCommand(_)) => {
            eprintln!("error cannot identify command");
            exit(2);
        }
        #[allow(unused_variables)]
        Result::Ok(arguments) => {
            // println!("Config is {:#?}", config);
            let param_todo_filename = arguments.config.todo_filename;
            let param_done_filename = arguments.config.done_filename;

            let todo_filename = param_todo_filename.or(config.todo_filename).expect(
                "todo filename is not present in config file and --todo-file parameter is missing",
            );
            let done_filename = param_done_filename.or(config.done_filename).expect(
                "done filename is not present in config file and --done-file parameter is missing",
            );
            run_app(todo_filename, done_filename, arguments.command)
        }
    }
}
