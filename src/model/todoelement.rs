use super::*;

#[derive(Debug, PartialEq)]
pub struct Uuid {
    uuid: u128,
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
        write!(
            f,
            "{}",
            match self {
                RecurrenceTimeUnit::B => 'b',
                RecurrenceTimeUnit::D => 'd',
                RecurrenceTimeUnit::M => 'm',
                RecurrenceTimeUnit::W => 'w',
                RecurrenceTimeUnit::Y => 'y',
            }
        )
    }
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
    #[allow(dead_code)]
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
                if *plus {
                    repr.push('+')
                }
                repr.push_str(&format!("{}", count));
                repr.push_str(&format!("{}", unit));
                write!(f, "rec:{}", repr)
            }
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

    pub fn project(str: &str) -> TodoElement {
        TodoElement::Project(String::from(str))
    }

    pub fn context(str: &str) -> TodoElement {
        TodoElement::Context(String::from(str))
    }

    pub fn text(str: &str) -> TodoElement {
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
