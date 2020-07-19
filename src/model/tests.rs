use super::*;

#[test]
pub fn parse_project(){
    let got: Result<super::TodoElement, ParsingError> = TodoElement::parse("+Project1");
    assert!(matches!(got, Result::Ok(super::TodoElement::Project(project_name)) if project_name == "Project1"));
}

#[test]
pub fn parse_project_fails_when_parsing_a_context(){
    let got: Result<super::TodoElement, ParsingError> = TodoElement::try_parse_project("@Site1");
    assert!(matches!(got, Result::Err(ParsingError{message:_})));
}

#[test]
pub fn parse_context(){
    let got: Result<super::TodoElement, ParsingError> = TodoElement::parse("@Site1");
    assert!(matches!(got, Result::Ok(super::TodoElement::Context(project_name)) if project_name == "Site1"));
}

#[test]
pub fn parse_text(){
    let got: Result<super::TodoElement, ParsingError> = TodoElement::parse("Site1");
    assert!(matches!(got, Result::Ok(super::TodoElement::Text(project_name)) if project_name == "Site1"));
}

pub fn parse_entry(){

}
