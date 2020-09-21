use super::*;

#[derive(Debug, PartialEq)]
pub enum Status {
    Done(Option<DateData>),
    Open,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        match self {
            Status::Done(Option::None) => { write!(f, "x ") },
            Status::Done(Option::Some(date)) => { write!(f, "x {} ", date) },
            _ => { write!(f, "") }
        }
    }
}
