/*
 * Model serialization tests
 */
#[allow(unused_imports)]
use crate::model::*;

#[test]
pub fn parse_project(){
    let got: TodoElement = TodoElement::parse("+Project1");
    assert!(matches!(got, TodoElement::Project(project_name) if project_name == "Project1"));
}

#[test]
pub fn parse_project_fails_when_parsing_a_context(){
    let got: Result<TodoElement, ParsingError> = TodoElement::try_parse_project("@Site1");
    assert!(matches!(got, Result::Err(ParsingError{message:_})));
}

#[test]
pub fn parse_context(){
    let got: TodoElement = TodoElement::parse("@Site1");
    assert!(matches!(got, TodoElement::Context(project_name) if project_name == "Site1"));
}

#[test]
pub fn parse_text(){
    let got: TodoElement = TodoElement::parse("Site1");
    assert!(matches!(got, TodoElement::Text(project_name) if project_name == "Site1"));
}

#[test]
pub fn parse_due(){
    let got: TodoElement = TodoElement::parse("due:2020-07-22");
    assert!(matches!(got, TodoElement::Due(DateData{year: y, month: m, day: d}) if y == 2020 && m == 7 && d == 22));
}

#[test]
pub fn parse_threshold(){
    let got: TodoElement = TodoElement::parse("t:2020-07-22");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Threshold(DateData{year: y, month: m, day: d}) if y == 2020 && m == 7 && d == 22));
}

#[test]
pub fn parse_due_falls_back_to_text_on_incorrect_format(){
    let got: TodoElement = TodoElement::parse("due:2020-x-22");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Text(t) if t == "due:2020-x-22"));
}

#[test]
pub fn parse_recurrence_plus_one_week(){
    let got: TodoElement = TodoElement::parse("rec:+1w");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Recurrence{plus, count, unit} if plus && count == 1 && unit == RecurrenceTimeUnit::W));
}

#[test]
pub fn parse_recurrence_10_days(){
    let got: TodoElement = TodoElement::parse("rec:10d");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Recurrence{plus, count, unit} if !plus && count == 10 && unit == RecurrenceTimeUnit::D));
}

#[test]
pub fn parse_recurrence_plus_10_years(){
    let got: TodoElement = TodoElement::parse("rec:+10y");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Recurrence{plus, count, unit} if plus && count == 10 && unit == RecurrenceTimeUnit::Y));
}

#[test]
pub fn parse_recurrence_5_months(){
    let got: TodoElement = TodoElement::parse("rec:5m");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Recurrence{plus, count, unit} if !plus && count == 5 && unit == RecurrenceTimeUnit::M));
}

#[test]
pub fn parse_recurrence_5_business_days(){
    let got: TodoElement = TodoElement::parse("rec:5b");
    println!("{:?}", got);
    assert!(matches!(got, TodoElement::Recurrence{plus, count, unit} if !plus && count == 5 && unit == RecurrenceTimeUnit::B));
}

#[test]
pub fn parse_entry(){
    match TodoEntry::parse("+Project1 @Site1 Foo bar due:2020-07-20 t:2020-07-26 rec:+1b") {
        Result::Ok(TodoEntry{
            parts: todo_elements,
            status: Status::Open,
            created_date: Option::None,
        }) => {
            for entry in vec!(
                TodoElement::project("Project1"), 
                TodoElement::context("Site1"),
                TodoElement::text("Foo bar"),
                TodoElement::Recurrence{plus:true, count:1, unit:RecurrenceTimeUnit::B},
                TodoElement::Threshold(DateData{year:2020, month:7, day:26}),
                TodoElement::Due(DateData{year:2020, month:7, day:20}),
            ) {
                if ! todo_elements.contains(&entry){
                    panic!(" entry {:?} not found", entry)
                }
            }
        },
        _ => panic!("error while parsing entry")
    }
}

#[test]
pub fn serialization_roundtrip(){
    let original = "+Project1 @Site1 Foo bar due:2020-07-20 t:2020-07-26 rec:+1b";
    let entry = TodoEntry::parse(original).unwrap();
    let text = format!("{}", entry);
    assert_eq!(text, original);
}

#[test]
pub fn serialization_roundtrip_2(){
    let original = "2020-05-15 window.requestAnimationFrame +background";
    let entry = TodoEntry::parse(original).unwrap();
    let text = format!("{}", entry);
    assert_eq!(text, original);
}