use super::Error;
use crate::args::ErrorType; 

impl std::convert::From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        return Error {
            message: format!("{}", error),
        };
    }
}

impl std::convert::From<&str> for Error {
    fn from(error: &str) -> Self {
        return Error {
            message: format!("{}", error),
        };
    }
}

impl std::convert::From<ErrorType> for Error {
    fn from(error: ErrorType) -> Self {
        match error {
            ErrorType::MissingArguments(unset_arguments) => {
                for unset_argument in unset_arguments {
                    eprintln!("error unset argument {}", unset_argument.long_form);
                }
                Error {
                    message: "missing arguments".into(),
                }
            }
            ErrorType::CannotIdentifyCommand(_) => Error {
                message: "error cannot identify command".into(),
            },
        }
    }
}