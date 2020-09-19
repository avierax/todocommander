mod config;
mod model;
mod tests;
mod args;
mod error_conversion;

use config::*;
use model::*;
use std::env;
use std::io::prelude::*;
use args::Command;
use args::parse_arguments;

#[derive(Debug)]
struct Error {
    message: String,
}

fn run_app(config: Config, command: Command) -> Result<(), Error> {
    let todo_str = std::fs::read_to_string(config.todo_filename.expect(
        "todo filename is not present in config file and --todo-file parameter is missing",
    ))?;
    let done_str = std::fs::read_to_string(config.done_filename.expect(
        "done filename is not present in config file and --done-file parameter is missing",
    ))?;
    let mut model = Model {
        todo_data: TodoData::parse(&todo_str).expect("error parsing todo file"),
        done_data: TodoData::parse(&done_str).expect("error parsing done file"),
    };
    model.execute(command).map_err(|e| Error {
        message: e.to_owned(),
    })
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
    run_app(config, arguments.command)
}
