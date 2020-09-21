mod tests;
mod todoentry;
mod todoelement;
mod datedata;
mod status;
mod tododata;

use chrono::prelude::*;
use common::*;
use std::fmt;
use crate::args::Command;

pub use todoentry::*;
pub use todoelement::*;
pub use datedata::*;
pub use status::*;
pub use tododata::*;

#[derive(Debug, PartialEq)]
pub struct Model {
    pub todo_data: TodoData,
    pub done_data: TodoData,
}

impl Model {
    pub fn execute(self: &mut Model, command: crate::args::Command) -> Result<(), &str> {
        match command {
            Command::Archive(offset) => Result::Ok({
                self.done_data
                    .entries
                    .push(self.todo_data.entries.remove(offset.into()))
            }),
            Command::Add(text) => TodoEntry::parse(&text)
                .and_then(|e| Result::Ok(self.todo_data.entries.push(e)))
                .map_err(|e| e.message),
            Command::Do(index) => Result::Ok({
                let date = Local::now().date();
                let year = date.year() as u16;
                let month = date.month() as u8;
                let day = date.day() as u8;
                self.todo_data.entries[index as usize].status =
                    Status::Done(Option::Some(DateData { year, month, day }))
            }),
            Command::Undo(index) => Result::Ok( self.todo_data.entries[index as usize].status = Status::Open ),
            Command::List => Result::Ok(
                for (i, entry) in self.todo_data.entries.iter().enumerate() {
                    println!("[{}] {}", i, entry)
                },
            ),
        }
    }
}

pub mod common {
    #[derive(Debug)]
    pub struct ParsingError {
        pub message: &'static str,
    }
}
