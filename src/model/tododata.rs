use super::*;

#[derive(Debug, PartialEq)]
pub struct TodoData {
    pub entries: Vec<TodoEntry>,
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
