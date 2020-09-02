mod model;
mod config;

use std::env;
use std::process::exit;

fn main() {
    if let Result::Err(unset_arguments) = config::parse_arguments(&mut env::args()) {
        for unset_argument in unset_arguments {
            eprintln!("error unset argument {}", unset_argument.long_form);
            exit(1);
        }
    }
    for argument in env::args() {
        println!("{}", argument);
    }
}
