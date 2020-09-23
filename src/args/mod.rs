#[derive(Debug)]
pub struct ArgsConfig {
    pub todo_filename: Option<String>,
    pub done_filename: Option<String>,
    pub help: bool,
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
    pub command: Option<Command>,
}

impl ArgsConfig {
    pub fn set_todo_filename(self: &mut ArgsConfig, value: String) {
        self.todo_filename = Option::Some(value);
    }

    pub fn set_done_filename(self: &mut ArgsConfig, value: String) {
        self.done_filename = Option::Some(value);
    }

    pub fn toggle_help(self: &mut ArgsConfig){
        self.help = true;
    }
}

pub enum ArgumentType {
    Parameterized(ArgumentDef),
    Flag(FlagDef),
}

pub struct ArgumentDef {
    pub long_form: &'static str,
    pub short_form: Option<&'static str>,
    pub help_message: &'static str,
    accessor: &'static dyn Fn(&mut ArgsConfig, String) -> (),
}

pub struct FlagDef {
    pub long_form: &'static str,
    pub short_form: Option<&'static str>,
    pub help_message: &'static str,
    accessor: &'static dyn Fn(&mut ArgsConfig) -> ()
}

fn find_arg_def<'a>(
    arg: &str,
    argument_defs_accessors: &'a [ArgumentType],
) -> Option<&'a ArgumentType>{
    for arg_type in argument_defs_accessors.iter() {
        // Reduce ciclomatic complexity
        match arg_type { 
            ArgumentType::Parameterized(arg_def) => if arg_def.long_form == arg || ( arg_def.short_form.is_some() && arg_def.short_form.unwrap() == arg ) {
                return Option::Some(arg_type);
            },
            ArgumentType::Flag(flag_def) => if flag_def.long_form == arg || ( flag_def.short_form.is_some() && flag_def.short_form.unwrap() == arg ) {
                return Option::Some(arg_type);
            }
        }
    }
    Option::None
}

const ARGUMENT_DEFS: &'static [ArgumentType] = &[
    ArgumentType::Flag(FlagDef {
        long_form: "--help",
        short_form: Option::None,
        help_message: "help",
        accessor: &ArgsConfig::toggle_help,
    }),
    ArgumentType::Parameterized(ArgumentDef {
        long_form: "--todo-file",
        short_form: Option::Some("-f"),
        help_message: "todo file",
        accessor: &ArgsConfig::set_todo_filename,
    }),
    ArgumentType::Parameterized(ArgumentDef {
        long_form: "--done-file",
        short_form: Option::Some("-d"),
        help_message: "done file",
        accessor: &ArgsConfig::set_done_filename,
    }),
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
        help: false,
    };

    let mut unprocessed_args: Vec<String> = Vec::new();
    while let Option::Some(arg) = args.next() {
        if let Option::Some(arg_type) = find_arg_def(&arg, &ARGUMENT_DEFS) {
            let argument = args.next();
            match arg_type {
                ArgumentType::Parameterized(arg_def) => (arg_def.accessor)(
                    &mut config,
                    argument.expect(&format!("argument {} not present", &arg)),
                ),
                ArgumentType::Flag(flag_def) => (flag_def.accessor)(&mut config)
            }
        } else {
            unprocessed_args.push(arg);
            while let Option::Some(arg) = args.next() {
                unprocessed_args.push(arg);
            }
            break;
        }
    }
    Result::Ok((config, unprocessed_args))
}

pub fn parse_command(command: &Vec<String>) -> Result<Option<Command>, ErrorType> {
    if command.len() > 0 {
        match command[0].as_str() {
            "add" => Result::Ok(Option::Some(Command::Add(command[1..].join(" ")))),
            "archive" => {
                let id = command[1].parse::<u16>().expect("error parsing task id");
                Result::Ok(Option::Some(Command::Archive(id)))
            }
            "do" => {
                let id = command[1].parse::<u16>().expect("error parsing task id");
                Result::Ok(Option::Some(Command::Do(id)))
            },
            "list" => Result::Ok(Option::Some(Command::List)),
            "undo" => {
                let id = command[1].parse::<u16>().expect("error parsing task id");
                Result::Ok(Option::Some(Command::Undo(id)))
            },
            _ => Result::Err(ErrorType::CannotIdentifyCommand(command.to_owned())),
        }
    } else {
        Result::Ok(Option::None)
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
