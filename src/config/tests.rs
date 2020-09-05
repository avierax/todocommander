#[test]
pub fn parse_erroneous_arguments(){
    use super::*;
    let parameters:Vec<String> = vec![String::from("--todo-file"),String::from("a")];
    let iter:&mut dyn Iterator<Item=String> = &mut parameters.iter().map(|s| String::from(s));
    if let Result::Err(ErrorType::MissingArguments(_)) =  parse_arguments(iter) {
        println!("This is ok");
    } else {
        panic!("This test failed");
    }
}