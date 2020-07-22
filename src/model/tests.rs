    #[test]
pub fn parse_project(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("+Project1");
    assert!(matches!(got, super::TodoElement::Project(project_name) if project_name == "Project1"));
}

#[test]
pub fn parse_project_fails_when_parsing_a_context(){
    use super::*;
    let got: Result<super::TodoElement, ParsingError> = TodoElement::try_parse_project("@Site1");
    assert!(matches!(got, Result::Err(ParsingError{message:_})));
}

#[test]
pub fn parse_context(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("@Site1");
    assert!(matches!(got, super::TodoElement::Context(project_name) if project_name == "Site1"));
}

#[test]
pub fn parse_text(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("Site1");
    assert!(matches!(got, super::TodoElement::Text(project_name) if project_name == "Site1"));
}

#[test]
pub fn parse_due(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("due:2020-07-22");
    assert!(matches!(got, super::TodoElement::Due(DateData{year: y, month: m, day: d}) if y == 2020 && m == 7 && d == 22));
}

#[test]
pub fn parse_threshold(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("t:2020-07-22");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Threshold(DateData{year: y, month: m, day: d}) if y == 2020 && m == 7 && d == 22));
}

#[test]
pub fn parse_due_falls_back_to_text_on_incorrect_format(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("due:2020-x-22");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Text(t) if t == "due:2020-x-22"));
}

#[test]
pub fn parse_recurrence_plus_one_week(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("rec:+1w");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Recurrence{plus, count, unit} if plus && count == 1 && unit == RecurrenceTimeUnit::W));
}

#[test]
pub fn parse_recurrence_10_days(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("rec:10d");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Recurrence{plus, count, unit} if !plus && count == 10 && unit == RecurrenceTimeUnit::D));
}

#[test]
pub fn parse_recurrence_plus_10_years(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("rec:+10y");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Recurrence{plus, count, unit} if plus && count == 10 && unit == RecurrenceTimeUnit::Y));
}

#[test]
pub fn parse_recurrence_5_months(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("rec:5m");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Recurrence{plus, count, unit} if !plus && count == 5 && unit == RecurrenceTimeUnit::M));
}

#[test]
pub fn parse_recurrence_5_business_days(){
    use super::*;
    let got: super::TodoElement = TodoElement::parse("rec:5b");
    println!("{:?}", got);
    assert!(matches!(got, super::TodoElement::Recurrence{plus, count, unit} if !plus && count == 5 && unit == RecurrenceTimeUnit::B));
}

#[test]
pub fn parse_entry(){
    use super::*;
    match TodoEntry::parse("+Project1 @Site1 Foo bar due:2020-07-20 t:2020-07-26 rec:+1b") {
        Result::Ok(TodoEntry{parts: todo_elements}) => {
            for entry in vec!(
                TodoElement::project("Project1"), 
                TodoElement::context("Site1"),
                TodoElement::text("Foo bar"),
                TodoElement::Recurrence{plus:true, count:1, unit:RecurrenceTimeUnit::B},
                TodoElement::Threshold(DateData{year:2020, month:7, day:26}),
                TodoElement::Due(DateData{year:2020, month:7, day:20}),
            ) {
                if ! todo_elements.contains(&entry){
                    panic!(" entry {:?} not found")
                }
            }
        },
        _ => panic!("error while parsing entry")
    }
}