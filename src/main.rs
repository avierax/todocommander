mod config;
mod model;
mod tests;
mod args;
mod error_conversion;
mod app;

use config::*;
use std::env;
use std::io::prelude::*;
use args::parse_arguments;

#[derive(Debug)]
pub struct Error {
    message: String,
}

fn read_configuration(mut config: &mut Config) {
    if let Option::Some(dirs) = directories::ProjectDirs::from("", "", "todocommander") {
        let mut path = dirs.config_dir().to_path_buf();
        path.push("todocommander.cfg");
        println!("{:?}", path);
        if let Result::Ok(mut f) = std::fs::File::open(path) {
            let mut file_content = String::new();
            let _result = f.read_to_string(&mut file_content);
            read_configuration_from_filecontent(&file_content, &mut config);
        }
    }
}

fn main() -> Result<(), Error> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    println!("{}", VERSION);
    let mut config = Config::new();
    read_configuration(&mut config);
    println!("{:?}", &config);
    let arguments = parse_arguments(&mut env::args())?;
    config = Config {
        todo_filename: arguments.config.todo_filename.or(config.todo_filename),
        done_filename: arguments.config.done_filename.or(config.done_filename),
    };
    let mut app = app::App::new(config)?;
    app.execute(arguments.command)
}
