#![allow(dead_code)]

pub mod tests;

use common::*;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct DateData{
    year:u16,
    month:u8,
    day:u8,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum RecurrenceTimeUnit {
    B, // business day
    D, // day
    M, // month
    W, // week
    Y, // year
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TodoElement {
    Project(String),
    Context(String),
    Text(String),
    Due(DateData),
    Threshold(DateData),
    Recurrence{
        plus:bool,
        count:u16,
        unit:RecurrenceTimeUnit,
    }
}

impl TodoElement {
    fn project(str: &str)-> TodoElement {
        TodoElement::Project(String::from(str))
    }

    fn context(str: &str)-> TodoElement {
        TodoElement::Context(String::from(str))
    }

    fn text(str: &str)-> TodoElement {
        TodoElement::Text(String::from(str))
    }

    fn create_prefix_parser(prefix:char, element_constructor: &'static dyn Fn(&str)->TodoElement)->Box<dyn Fn(&str)-> Result<TodoElement, ParsingError>> {
        Box::new(move |input: &str| {
            if let Some(data) = input.strip_prefix(prefix) {
                Result::Ok(element_constructor(data))
            } else {
                Result::Err(ParsingError{message:"error parsing entity"})
            }
        })
    }
    
    pub fn try_parse_project(input:&str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('+', &TodoElement::project)(input)
    }
    
    fn try_parse_context(input:&str) -> Result<TodoElement, ParsingError> {
        TodoElement::create_prefix_parser('@', &TodoElement::context)(input)
    }
    
    fn try_parse_text(input:&str) -> Result<TodoElement, ParsingError> {
        Result::Ok(TodoElement::text(input))
    }

    fn create_date_parser(prefix: &'static str, constructor:&'static dyn Fn(DateData)->TodoElement)->Box<dyn Fn(&str)-> Result<TodoElement, ParsingError>> {
        Box::new(move |input: &str| {
            if let Some(date_str) = input.strip_prefix(prefix) {
                let x:Vec<&str> = date_str.split('-').collect();
                match (x.get(0),x.get(1),x.get(2)) {
                    (Some(year_str), Some(month_str), Some(day_str)) => {
                        Result::Ok(
                            constructor(
                                DateData{
                                    year: year_str.parse::<u16>().map_err(|_|{ParsingError{message:"error parsing year"}})?,
                                    month: month_str.parse::<u8>().map_err(|_|{ParsingError{message:"error parsing month"}})?,
                                    day: day_str.parse::<u8>().map_err(|_|{ParsingError{message:"error parsing day"}})?
                                }
                            )
                        )
                    },
                    _ => Result::Err(ParsingError{message:"error parsing date"})
                }
            } else {
                Result::Err(ParsingError{message:"error parsing entity"})
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
            Result::Ok(TodoElement::Recurrence{ 
                plus: rec_str.starts_with('+'),
                count: rec_str.trim_start_matches('+').trim_end_matches(x).parse::<u16>().map_err(|_|{ParsingError{message:"error parsing recurrence"}})?,
                unit: match rec_str.chars().last() {
                    Some(x) if x == 'd' => Result::Ok(RecurrenceTimeUnit::D),
                    Some(x) if x == 'b' => Result::Ok(RecurrenceTimeUnit::B),
                    Some(x) if x == 'm' => Result::Ok(RecurrenceTimeUnit::M),
                    Some(x) if x == 'w' => Result::Ok(RecurrenceTimeUnit::W),
                    Some(x) if x == 'y' => Result::Ok(RecurrenceTimeUnit::Y),
                    _ => Result::Err(ParsingError{message:"error parsing recurrence"})
                }?
            })
        } else {
            Result::Err(ParsingError{message:"error parsing entity"})
        }
    }
    
    pub fn parse(input: &str) -> Result<TodoElement, ParsingError> {
        let parsers = [
            TodoElement::try_parse_project, 
            TodoElement::try_parse_context, 
            TodoElement::try_parse_due,
            TodoElement::try_parse_threshold,
            TodoElement::try_parse_recurrence,
            TodoElement::try_parse_text,
        ];
        let mut iterator = parsers.iter();
        let mut last_error: Option<Result<TodoElement, ParsingError>> = Option::None;
        while let Some(parser) = iterator.next() {
            match parser(input) {
                parse_result @ Ok(_) => return parse_result,
                error @ Err(_) => last_error = Option::Some(error),
            }
        }
        last_error.unwrap()
    }
}

struct TodoEntry {
    parts:Vec<TodoElement>
}

impl TodoEntry {
    pub fn parse(data:&str) -> Result<TodoEntry, ParsingError>{
        let mut result = TodoEntry { parts : Vec::new() };
        for split in data.split_whitespace() {
            result.parts.push(TodoElement::parse(split).unwrap());
        }
        Result::Ok(result)
    }
}

struct TodoData {
    entries:Vec<TodoEntry>
}

impl TodoData {
    pub fn parse(data:&str) -> Result<TodoData, ParsingError> {
        let mut result = TodoData{
            entries:Vec::new()
        };
        for line in data.lines() {
            result.entries.push(TodoEntry::parse(line).expect("error parsing"));
        }
        Result::Ok(
            result
        )
    }
}

pub mod common {
    #[derive(Debug)]
    pub struct ParsingError {
        pub message: &'static str
    }
}