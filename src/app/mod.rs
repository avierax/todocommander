use crate::config::Config;
use crate::model::Model;
use crate::model::TodoData;
use crate::args::Command;
use crate::Error;

pub struct App {
    config: Config,
    model: crate::model::Model,
}

impl App {
    pub fn new(config: Config) -> Result<Self, Error> {
        match &config {
            Config {
                todo_filename: Option::Some(todo_filename),
                done_filename: Option::Some(done_filename),
                ..
            } => {
                let todo_str = std::fs::read_to_string(&todo_filename)?;
                let done_str = std::fs::read_to_string(&done_filename)?;
                Result::Ok(App {
                    config,
                    model: Model {
                        todo_data: TodoData::parse(&todo_str).expect("error parsing todo file"),
                        done_data: TodoData::parse(&done_str).expect("error parsing done file"),
                    },
                })
            },
            _ => {
               Result::Err(Error{message:"cannot read model".into()}) 
            }
        }
    }

    pub fn execute(&mut self, command: Command)->Result<(), Error> {
        self.model.execute(command).map_err(|e| Error {
            message: e.to_owned(),
        })
    }

    pub fn save_model(&mut self) -> Result<(), Error> {
        let mut todo_data_str = String::new();
        for entry in &self.model.todo_data.entries {
            todo_data_str.push_str(&format!("{}\n", entry));
        }
        let o = &self.config.todo_filename.as_ref();
        let p : String = o.unwrap().clone();
        std::fs::write(p, todo_data_str)?;
        Result::Ok(())
    }
}