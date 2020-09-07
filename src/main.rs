mod model;
mod config;

use std::env;
use std::process::exit;
use config::ErrorType;

fn main() {
    match config::parse_arguments(&mut env::args()) {
        Result::Err(ErrorType::MissingArguments(unset_arguments)) =>  {
            for unset_argument in unset_arguments {
                eprintln!("error unset argument {}", unset_argument.long_form);
                exit(1);
            }
        },
        Result::Err(ErrorType::CannotIdentifyCommand(_)) => {
            eprintln!("error cannot identify command");
            exit(2);
        }
        #[allow(unused_variables)]
        Result::Ok(arguments) => {
            // println!("Config is {:#?}", config);
            let todo_file = std::fs::File::open(arguments.config.todo_filename.unwrap());
            let done_file = std::fs::File::open(arguments.config.done_filename.unwrap());
        }
    }
}
