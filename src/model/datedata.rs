use super::*;

#[derive(Debug, PartialEq)]
pub struct DateData {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl DateData {
    pub fn parse(date_str: &str) -> Result<DateData, ParsingError> {
        let x: Vec<&str> = date_str.split('-').collect();
        match (x.get(0), x.get(1), x.get(2)) {
            (Some(year_str), Some(month_str), Some(day_str)) => Result::Ok(DateData {
                year: year_str.parse::<u16>().map_err(|_| ParsingError {
                    message: "error parsing year",
                })?,
                month: month_str.parse::<u8>().map_err(|_| ParsingError {
                    message: "error parsing month",
                })?,
                day: day_str.parse::<u8>().map_err(|_| ParsingError {
                    message: "error parsing day",
                })?,
            }),
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
