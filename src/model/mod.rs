#![allow(dead_code)]
pub mod model {
    use super::common::*;

    enum TodoElement {
        Project(String),
        Text(String),
    }

    fn try_parse_project(input:&str) -> Result<TodoElement, ParsingError> {
        if let Some(project_name) = input.strip_prefix('+') {
            Result::Ok(TodoElement::Project(String::from(project_name)))
        } else {
            Result::Err(ParsingError{message:"error parsing project"})
        }
    }

    fn try_parse_context(input:&str) -> Result<TodoElement, ParsingError> {
        if let Some(project_name) = input.strip_prefix('@') {
            Result::Ok(TodoElement::Project(String::from(project_name)))
        } else {
            Result::Err(ParsingError{message:"error parsing context"})
        }
    }

    fn try_parse_text(input:&str) -> Result<TodoElement, ParsingError> {
        Result::Ok(TodoElement::Text(String::from(input)))
    }

    fn parse_part(input: &str) -> Result<TodoElement, ParsingError> {
        let parsers = vec![
            try_parse_project, 
            try_parse_context, 
            try_parse_text
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

    struct TodoEntry {
        parts:Vec<TodoElement>
    }

    impl TodoEntry {
        pub fn parse(data:&str) -> Result<TodoEntry, ParsingError>{
            let mut result = TodoEntry { parts : Vec::new() };
            for split in data.split_whitespace() {
                result.parts.push(parse_part(split).unwrap());
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

}

pub mod common {
    #[derive(Debug)]
    pub struct ParsingError {
        pub message: &'static str
    }
}