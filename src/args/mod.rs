#[derive(Debug)]
pub struct ArgsConfig {
    pub todo_filename: Option<String>,
    pub done_filename: Option<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Command {
    Add (String),
    Archive (u16),
    Do(u16),
    List,
    Undo(u16),
}

#[derive(Debug)]
pub struct Arguments {
    pub config: ArgsConfig,
    pub command: Command,
}

impl ArgsConfig {
    pub fn set_todo_filename(self: &mut ArgsConfig, value: String) {
        self.todo_filename = Option::Some(value);
    }

    pub fn set_done_filename(self: &mut ArgsConfig, value: String) {
        self.done_filename = Option::Some(value);
    }
}

pub struct ArgumentDef {
    pub long_form: &'static str,
    pub short_form: Option<&'static str>,
    pub help_message: &'static str,
    pub mandatory: bool,
    accessor: &'static dyn Fn(&mut ArgsConfig, String) -> (),
}

fn find_arg_def<'a>(
    arg: &str,
    argument_defs_accessors: &'a [ArgumentDef],
) -> Option<(usize, &'a ArgumentDef)> {
    for (i, arg_def) in argument_defs_accessors.iter().enumerate() {
        if arg_def.long_form == arg || arg_def.short_form.unwrap() == arg
        {
            return Option::Some((i, arg_def));
        }
    }
    Option::None
}

const ARGUMENT_DEFS: &'static [ArgumentDef] = &[
    ArgumentDef {
        long_form: "--help",
        short_form: Option::None,
        help_message: "help",
        mandatory: false,
        accessor: &ArgsConfig::set_todo_filename,
    },
    ArgumentDef {
        long_form: "--todo-file",
        short_form: Option::Some("-f"),
        help_message: "todo file",
        mandatory: false,
        accessor: &ArgsConfig::set_todo_filename,
    },
    ArgumentDef {
        long_form: "--done-file",
        short_form: Option::Some("-d"),
        help_message: "done file",
        mandatory: false,
        accessor: &ArgsConfig::set_done_filename,
    },
];

pub enum ErrorType {
    MissingArguments(Vec<&'static ArgumentDef>),
    CannotIdentifyCommand(Vec<String>),
}

pub fn parse_config(
    args: &mut dyn Iterator<Item = String>,
) -> Result<(ArgsConfig, /*unprocessed args*/ Vec<String>), ErrorType> {
    let mut config = ArgsConfig {
        todo_filename: Option::None,
        done_filename: Option::None,
    };
    let mut must_include_args: Vec<bool> = Vec::new();
    for arg_def in ARGUMENT_DEFS.iter() {
        must_include_args.push(arg_def.mandatory);
    }

    let mut unprocessed_args: Vec<String> = Vec::new();
    while let Option::Some(arg) = args.next() {
        if let Option::Some((i, arg_def)) = find_arg_def(&arg, &ARGUMENT_DEFS) {
            let argument = args.next();
            (arg_def.accessor)(
                &mut config,
                argument.expect(&format!("argument {} not present", &arg)),
            );
            must_include_args[i] = false;
        } else {
            unprocessed_args.push(arg);
            while let Option::Some(arg) = args.next() {
                unprocessed_args.push(arg);
            }
            break;
        }
    }

    let mut unset_arguments: Vec<&ArgumentDef> = Vec::new();

    for (i, arg_def) in ARGUMENT_DEFS.iter().enumerate() {
        if must_include_args[i] {
            unset_arguments.push(&arg_def);
        }
    }

    if !unset_arguments.is_empty() {
        Result::Err(ErrorType::MissingArguments(unset_arguments))
    } else {
        Result::Ok((config, unprocessed_args))
    }
}

pub fn parse_command(command: &Vec<String>) -> Result<Command, ErrorType> {
    match command[0].as_str() {
        "add" => Result::Ok(Command::Add(command[1..].join(" "))),
        "archive" => {
            let id = command[1].parse::<u16>().expect("error parsing task id");
            Result::Ok(Command::Archive(id))
        }
        "do" => {
            let id = command[1].parse::<u16>().expect("error parsing task id");
            Result::Ok(Command::Do(id))
        },
        "list" => Result::Ok(Command::List),
        "undo" => {
            let id = command[1].parse::<u16>().expect("error parsing task id");
            Result::Ok(Command::Undo(id))
        },
        _ => Result::Err(ErrorType::CannotIdentifyCommand(command.to_owned())),
    }
}

pub fn parse_arguments(args: &mut dyn Iterator<Item = String>) -> Result<Arguments, ErrorType> {
    parse_config(&mut args.skip(1)).and_then(|config_and_rest| {
        parse_command(&config_and_rest.1).map(|command| Arguments {
            config: config_and_rest.0,
            command,
        })
    })
}
