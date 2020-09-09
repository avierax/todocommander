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

#[test]
pub fn parse_arguments_1() {
    use super::*;
    let parameters: Vec<&str> = vec!["--done-file", "a", "--todo-file", "b", "do", "36"];
    let iter: &mut dyn Iterator<Item = String> = &mut parameters.iter().map(|s| String::from(*s));
    assert!(matches!(
        parse_arguments(iter),
        Result::Ok(Arguments {
            config: Config {
                todo_filename: Option::Some(b),
                done_filename: Option::Some(a),
            },
            command: Command::Do(36),
        }) if a == "a" && b == "b"
    ));
}

#[test]
pub fn parse_arguments_2() {
    use super::*;
    let parameters = vec!["--done-file", "a", "--todo-file", "b", "add", "foo bar", "baz"];
    let iter: &mut dyn Iterator<Item = String> = &mut parameters.iter().map(|s| String::from(*s));
    assert!(matches!(
        parse_arguments(iter),
        Result::Ok(Arguments {
            config: Config {
                todo_filename: Option::Some(b),
                done_filename: Option::Some(a),
            },
            command: Command::Add(text),
        }) if a == "a" && b == "b" && text == "foo bar baz"
    ));
}

#[test]
pub fn parse_config_unprocessed_args() {
    use super::*;
    let parameters: Vec<&str> = vec!["--done-file", "a", "--todo-file", "b", "do", "36"];
    let iter: &mut dyn Iterator<Item = String> = &mut parameters.iter().map(|s| String::from(*s));
    match parse_config(iter) {
        Result::Ok((_, unprocessed_args)) if unprocessed_args == vec!["do", "36"] => (),
        _ => panic!("test failed"),
    }
}
