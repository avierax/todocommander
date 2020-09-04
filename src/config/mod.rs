#[derive(Debug)]
pub struct Config {
    pub todo_filename: Option<String>,
    pub done_filename: Option<String>,
}

#[allow(dead_code)]
pub enum Command {
    Do{
        id:u16,
    },
    Undo{
        id:u16,
    },
    List
}

pub struct Arguments {
    pub config: Config,
    pub command: Command,
}

impl Arguments {
    pub fn set_todo_filename(self:&mut Arguments, value: String){
        self.config.todo_filename = Option::Some(value);
    }

    pub fn set_done_filename(self:&mut Arguments, value: String){
        self.config.done_filename = Option::Some(value);
    }
}

#[derive(Debug)]
pub struct ArgumentDef {
    pub long_form: &'static str,
    pub short_form: Option<&'static str>,
    pub help_message: &'static str,
    pub mandatory: bool,
}

struct ArgumentDefAccessor {
    argument_def: ArgumentDef,
    accessor: &'static dyn Fn(&mut Arguments, String)->(),
}

fn find_arg_def<'a>(arg:&str, argument_defs_accessors:&'a [ArgumentDefAccessor]) -> Option<(usize,&'a ArgumentDefAccessor)> {
    for (i,arg_def) in argument_defs_accessors.iter().enumerate() {
        if arg_def.argument_def.long_form == arg || arg_def.argument_def.short_form.unwrap() == arg {
            return Option::Some((i,arg_def));
        }
    }
    Option::None
}

const ARGUMENT_DEFS_ACCESSORS:&'static [ArgumentDefAccessor] = &[
    ArgumentDefAccessor {
        argument_def: ArgumentDef {
            long_form: "--todo-file",
            short_form: Option::Some("-f"),
            help_message: "todo file",
            mandatory: true,
        },
        accessor: &Arguments::set_todo_filename,
    },
    ArgumentDefAccessor {
        argument_def: ArgumentDef {
            long_form: "--done-file",
            short_form: Option::Some("-d"),
            help_message: "done file",
            mandatory: true,
        },
        accessor: &Arguments::set_done_filename
    }
];

pub fn parse_arguments(args:&mut dyn Iterator<Item=String>)->Result<Arguments, Vec<&ArgumentDef>> {
    let mut arguments = Arguments {
        config:Config { 
            todo_filename: Option::None,
            done_filename: Option::None,
        },
        command: Command::List,
    };

    let mut must_include_args:Vec<bool> = Vec::new();
    for (i, arg_def_acc) in ARGUMENT_DEFS_ACCESSORS.iter().enumerate() {
        let argument_def: &ArgumentDef = &arg_def_acc.argument_def;
        if argument_def.mandatory {
            must_include_args[i] = true;
        }
    }
    
    while let Option::Some(arg) = args.next() {
        match find_arg_def(&arg, &ARGUMENT_DEFS_ACCESSORS) {
            Option::Some((i,arg_def)) => {
                let argument = args.next();
                (arg_def.accessor)(&mut arguments, argument.expect(&format!("argument {} not present", &arg)));
                must_include_args[i]=false;
            },
            _ =>  ()
        }
    }

    let mut unset_arguments:Vec<&ArgumentDef> = Vec::new();

    for (i, arg_def_acc) in ARGUMENT_DEFS_ACCESSORS.iter().enumerate() {
        if must_include_args[i] {
            unset_arguments.push(&arg_def_acc.argument_def);
        }
    }

    if unset_arguments.len() > 0 {
        Result::Err(unset_arguments)
    } else {
        Result::Ok(arguments)
    }
}