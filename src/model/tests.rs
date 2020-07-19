use super::TodoElement;
use super::try_parse_project;
use super::try_parse_context;
use super::common::ParsingError;
#[test]
pub fn parse_project(){
    let got: Result<TodoElement, ParsingError> = try_parse_project("+Project1");
    assert!(matches!(got, Result::Ok(TodoElement::Project(project_name)) if project_name == "Project1"));
}

#[test]
pub fn parse_project_fails_when_parsing_a_context(){
    let got: Result<TodoElement, ParsingError> = try_parse_project("@Site1");
    assert!(matches!(got, Result::Err(ParsingError{message:_})));
}

#[test]
pub fn parse_context(){
    let got: Result<TodoElement, ParsingError> = try_parse_context("@Site1");
    assert!(matches!(got, Result::Ok(TodoElement::Context(project_name)) if project_name == "Site1"));
}
