mod model;
mod config;

use std::env;
use std::process::exit;

fn main() {
    match config::parse_config(&mut env::args()) {
        Result::Err(unset_arguments) =>  {
            for unset_argument in unset_arguments {
                eprintln!("error unset argument {}", unset_argument.long_form);
                exit(1);
            }
        },
        Result::Ok(config) => {
            // println!("Config is {:#?}", config);
            let todo_file = std::fs::File::open(config.todo_filename.unwrap());
            let done_file = std::fs::File::open(config.done_filename.unwrap());
        }
    }
}
