mod model;
mod config;

use std::env;
use std::process::exit;

fn main() {
    match config::parse_arguments(&mut env::args()) {
        Result::Err(unset_arguments) =>  {
            for unset_argument in unset_arguments {
                eprintln!("error unset argument {}", unset_argument.long_form);
                exit(1);
            }
        },
        Result::Ok(config) => {
            println!("Config is {:#?}", config);
        }
    }
    for argument in env::args() {
        println!("{}", argument);
    }
}
