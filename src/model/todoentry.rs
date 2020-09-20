use super::*;

#[derive(Debug, PartialEq)]
pub struct TodoEntry {
    pub status: Status,
    pub created_date: Option<DateData>,
    pub parts: Vec<TodoElement>,
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

    fn try_parse_status(_data: &str) -> (Status, usize) {
        (Status::Open, 13)
    }

    pub fn parse(data: &str) -> Result<TodoEntry, ParsingError> {
        let mut parts: Vec<TodoElement> = Vec::new();
        let mut split_parts: Vec<&str> = data.split_whitespace().collect();
        let mut created_date = Option::None;
        let mut status = Status::Open;
        if !split_parts.is_empty() {
            if split_parts[0].starts_with("x") {
                status = Status::Done(DateData::parse(split_parts[1]).ok())
            }   
            if let Status::Done(Option::Some(_)) = status {
                split_parts = split_parts[2..].into(); // skip two
            }
            created_date = DateData::parse(split_parts[0]).ok();
            if let Option::Some(_) = created_date {
                split_parts = split_parts[1..].into();
            }
        }
        for split in split_parts.iter() {
            TodoEntry::push(&mut parts, TodoElement::parse(split));
        }
        Result::Ok(TodoEntry {
            status,
            created_date,
            parts,
        })
    }
}

impl fmt::Display for TodoEntry {
    fn fmt(self: &TodoEntry, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.status, 
            self.parts
                .iter()
                .map(|p| { format!("{}", p) })
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
