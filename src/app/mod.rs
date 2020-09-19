use crate::config::Config;
use crate::model::Model;
use crate::model::TodoData;
use crate::args::Command;
use crate::Error;

pub struct App {
    model: crate::model::Model,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        let todo_str = std::fs::read_to_string(config.todo_filename.expect(
            "todo filename is not present in config file and --todo-file parameter is missing",
        ))?;
        let done_str = std::fs::read_to_string(config.done_filename.expect(
            "done filename is not present in config file and --done-file parameter is missing",
        ))?;
        Result::Ok(App {
            model: Model {
                todo_data: TodoData::parse(&todo_str).expect("error parsing todo file"),
                done_data: TodoData::parse(&done_str).expect("error parsing done file"),
            },
        })
    }

    pub fn execute(&mut self, command: Command)->Result<(), Error> {
        self.model.execute(command).map_err(|e| Error {
            message: e.to_owned(),
        })
    }
}
