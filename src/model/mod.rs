#![allow(dead_code)]

pub mod tests;

use common::*;
use crate::config::Command;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct DateData {
    year: u16,
    month: u8,
    day: u8,
}

impl DateData {
    pub fn parse(date_str:&str)->Result<DateData, ParsingError>{
        let x: Vec<&str> = date_str.split('-').collect();
        match (x.get(0), x.get(1), x.get(2)) {
            (Some(year_str), Some(month_str), Some(day_str)) => {
                Result::Ok(DateData {
                    year: year_str.parse::<u16>().map_err(|_| ParsingError {
                        message: "error parsing year",
                    })?,
                    month: month_str.parse::<u8>().map_err(|_| ParsingError {
                        message : "error parsing month",
                    })?,
                    day: day_str.parse::<u8>().map_err(|_| ParsingError {
                        message: "error parsing day",
                    })?,
                })
            }
            _ => Result::Err(ParsingError {
                message: "error parsing date",
            }),
        }
    }
}

impl fmt::Display for DateData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

#[derive(Debug, PartialEq)]
pub enum RecurrenceTimeUnit {
    B, // business day
    D, // day
    M, // month
    W, // week
    Y, // year
}

impl fmt::Display for RecurrenceTimeUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                RecurrenceTimeUnit::B => 'b',
                RecurrenceTimeUnit::D => 'd',
                RecurrenceTimeUnit::M => 'm',
                RecurrenceTimeUnit::W => 'w',
                RecurrenceTimeUnit::Y => 'y'
            }
        )
    }
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Done(Option<DateData>),
    Open
}

#[derive(Debug, PartialEq)]
pub struct Uuid {
    uuid:u128
}

#[derive(Debug, PartialEq)]
pub enum TodoElement {
    Context(String),
    Due(DateData),
    Project(String),
    Recurrence {
        plus: bool,
        count: u16,
        unit: RecurrenceTimeUnit,
    },
    Text(String),
    Threshold(DateData),
    Uuid(Uuid),
}

impl fmt::Display for TodoElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TodoElement::Context(context) => write!(f, "@{}", context),
            TodoElement::Due(date_data) => write!(f, "due:{}", date_data),
            TodoElement::Project(project) => write!(f, "+{}", project),
            TodoElement::Recurrence { plus, count, unit } => {
                let mut repr = String::new();
                if *plus { repr.push('+') }
                repr.push_str(&format!("{}", count));
                repr.push_str(&format!("{}", unit));
                write!(f, "rec:{}", repr)
            },
            TodoElement::Text(text) => write!(f, "{}", text),
            TodoElement::Threshold(date_data) => write!(f, "t:{}", date_data),
            TodoElement::Uuid(_) => panic!("not implemented"),
        }
    }
}

impl TodoElement {
    pub fn merge_texts(element1: TodoElement, element2: TodoElement) -> TodoElement {
        match (element1, element2) {
            (TodoElement::Text(text1), TodoElement::Text(text2)) => {
                TodoElement::Text(text1 + " " + &text2)
            }
            tuple => panic!("only text element can be merged found {:?}", tuple),
        }
    }

    fn project(str: &str) -> TodoElement {
        TodoElement::Project(String::from(str))
    }

    fn context(str: &str) -> TodoElement {
        TodoElement::Context(String::from(str))
    }

    fn text(str: &str) -> TodoElement {
        TodoElement::Text(String::from(str))
    }

    pub fn is_text(self: &TodoElement) -> bool {
        if let TodoElement::Text(_) = self {
            true
        } else {
            false
        }
    }

    fn create_prefix_parser(
        prefix: char,
        element_constructor: &'static dyn Fn(&str) -> TodoElement,
    ) -> Box<dyn Fn(&str) -> Result<TodoElement, ParsingError>> {
        Box::new(move |input: &str| {
            if let Some(data) = input.strip_prefix(prefix) {
                Result::Ok(element_constructor(data))
            } else {
                Result::Err(ParsingError {
                    message: "error parsing entity",
                })
            }
        })
    }

    pub fn try_parse_project(input: &str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('+', &TodoElement::project)(input)
    }

    fn try_parse_context(input: &str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('@', &TodoElement::context)(input)
    }

    fn create_date_parser(
        prefix: &'static str,
        constructor: &'static dyn Fn(DateData) -> TodoElement,
    ) -> Box<dyn Fn(&str) -> Result<TodoElement, ParsingError>> {
        Box::new(move |input: &str| {
            if let Some(date_str) = input.strip_prefix(prefix) {
                DateData::parse(date_str).map(constructor)
            } else {
                Result::Err(ParsingError {
                    message: "error parsing entity",
                })
            }
        })
    }

    fn try_parse_due(input: &str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_date_parser("due:", &TodoElement::Due)(input)   
    }
    fn try_parse_threshold(input: &str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_date_parser("t:", &TodoElement::Threshold)(input)
    }

    fn try_parse_recurrence(input: &str) -> Result<TodoElement, ParsingError> {
        if let Some(rec_str) = input.strip_prefix("rec:") {
            let x: &'static [_] = &['d', 'b', 'm', 'w', 'y'];
            Result::Ok(TodoElement::Recurrence {
                plus: rec_str.starts_with('+'),
                count: rec_str
                    .trim_start_matches('+')
                    .trim_end_matches(x)
                    .parse::<u16>()
                    .map_err(|_| ParsingError {
                        message: "error parsing recurrence",
                    })?,
                unit: match rec_str.chars().last() {
                    Some(x) if x == 'd' => Result::Ok(RecurrenceTimeUnit::D),
                    Some(x) if x == 'b' => Result::Ok(RecurrenceTimeUnit::B),
                    Some(x) if x == 'm' => Result::Ok(RecurrenceTimeUnit::M),
                    Some(x) if x == 'w' => Result::Ok(RecurrenceTimeUnit::W),
                    Some(x) if x == 'y' => Result::Ok(RecurrenceTimeUnit::Y),
                    _ => Result::Err(ParsingError {
                        message: "error parsing recurrence",
                    }),
                }?,
            })
        } else {
            Result::Err(ParsingError {
                message: "error parsing entity",
            })
        }
    }
    pub fn parse(input: &str) -> TodoElement {
        for parser in &[
            TodoElement::try_parse_project,
            TodoElement::try_parse_context,
            TodoElement::try_parse_due,
            TodoElement::try_parse_threshold,
            TodoElement::try_parse_recurrence,
        ] {
            match parser(input) {
                Ok(element) => return element,
                _ => (), // do nothing with the error, they only exist as a form of documentation and to support unit testing
            }
        }
        TodoElement::text(input)
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct TodoEntry {
    status: Status,
    created_date: Option<DateData>,
    parts: Vec<TodoElement>,
}

impl TodoEntry {
    pub fn push(parts: &mut Vec<TodoElement>, element: TodoElement) {
        if element.is_text() {
            match parts.last() {
                Option::Some(last) if last.is_text() => {
                    let len = parts.len();
                    let last = parts.remove(len - 1);
                    let new = TodoElement::merge_texts(last, element);
                    parts.push(new);
                    return;
                }
                _ => (),
            }
        }
        parts.push(element);
    }
}

impl TodoEntry {
    fn try_parse_status(_data: &str) -> (Status, usize){
        (Status::Open, 13)
    }

    pub fn parse(data: &str) -> Result<TodoEntry, ParsingError> {
        let mut parts:Vec<TodoElement> = Vec::new();
        let mut split_parts: Vec<&str> = data.split_whitespace().collect();
        let status = if split_parts[0].starts_with("x") {
            Status::Done(DateData::parse(split_parts[1]).ok())
        } else {
            Status::Open
        };
        if let Status::Done(Option::Some(_)) = status {
            split_parts = split_parts[2..].into(); // skip two
        }
        let created_date = DateData::parse(split_parts[0]).ok();
        if let Option::Some(_) = created_date {
            split_parts = split_parts[1..].into();
        }
        for split in split_parts.iter() {
            TodoEntry::push(&mut parts, TodoElement::parse(split));
        }
        Result::Ok(TodoEntry { status, created_date, parts })
    }
}

impl fmt::Display for TodoEntry {
    fn fmt(self: &TodoEntry, f:&mut fmt::Formatter<'_>)-> fmt::Result {
        write!(f, "{}", self.parts.iter().map(|p|{ format!("{}", p) }).collect::<Vec<String>>().join(" "))
    }
}

#[derive(PartialEq)]
#[derive(Debug)]
struct TodoData {
    entries: Vec<TodoEntry>,
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Model {
    todo_data: TodoData,
    done_data: TodoData,
}

impl Model {
    pub fn execute(self: &mut Model, command:crate::config::Command) -> Result<(), &str>{
        match command {
            Command::Archive(offset) => {
                self.done_data.entries.push(self.todo_data.entries.remove(offset.into()));
                Result::Ok(())
            },
            _ => Result::Err("Operation not implemented")
        }
    }
}

impl TodoData {
    pub fn parse(data: &str) -> Result<TodoData, ParsingError> {
        let mut result = TodoData {
            entries: Vec::new(),
        };
        for line in data.lines() {
            result
                .entries
                .push(TodoEntry::parse(line).expect("error parsing"));
        }
        Result::Ok(result)
    }
}

pub mod common {
    #[derive(Debug)]
    pub struct ParsingError {
        pub message: &'static str,
    }
}
