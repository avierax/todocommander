pub struct Config {
    todo_filename: String,
    done_filename: String
}

struct ArgumentDef {
    long_form: &'static str,
    short_form: Option<&'static str>,
    help_message: String,
    mandatory: bool
}