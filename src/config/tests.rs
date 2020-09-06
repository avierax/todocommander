#[test]
pub fn parse_erroneous_arguments_1() {
    use super::*;
    let parameters: Vec<&str> = vec!["--todo-file", "a"];
    let iter: &mut dyn Iterator<Item = String> = &mut parameters.iter().map(|s| String::from(*s));
    match parse_arguments(iter) {
        Result::Err(ErrorType::MissingArguments(missing_arguments_vector)) => assert!(matches!(
            missing_arguments_vector
                .iter()
                .find(|arg_def| match arg_def {
                    ArgumentDef {
                        long_form: "--done-file",
                        ..
                    } => true,
                    _ => false,
                }),
            Option::Some(_)
        )),
        _ => {
            panic!("This test failed");
        }
    }
}

#[test]
pub fn parse_erroneous_arguments_2() {
    use super::*;
    let parameters: Vec<&str> = vec!["--done-file", "a"];
    let iter: &mut dyn Iterator<Item = String> = &mut parameters.iter().map(|s| String::from(*s));
    match parse_arguments(iter) {
        Result::Err(ErrorType::MissingArguments(missing_arguments_vector)) => assert!(matches!(
            missing_arguments_vector
                .iter()
                .find(|arg_def| match arg_def {
                    ArgumentDef {
                        long_form: "--todo-file",
                        ..
                    } => true,
                    _ => false,
                }),
            Option::Some(_)
        )),
        _ => {
            panic!("This test failed");
        }
    }
}
