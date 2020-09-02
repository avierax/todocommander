use std::env::Args;
use std::collections::hash_set::HashSet;

pub struct Config {
    todo_filename: Option<String>,
    done_filename: Option<String>,
}

impl Config {
    pub fn set_todo_filename(self:&mut Config, value: String){
        self.todo_filename = Option::Some(value);
    }

    pub fn set_done_filename(self:&mut Config, value: String){
        self.done_filename = Option::Some(value);
    }
}


#[derive(PartialEq)]
#[derive(Hash)]
#[derive(Eq)]
#[derive(Debug)]
pub struct ArgumentDef {
    pub long_form: &'static str,
    pub short_form: Option<&'static str>,
    pub help_message: &'static str,
    pub mandatory: bool,
}

struct ArgumentDefAccessor {
    argument_def: ArgumentDef,
    accessor: &'static dyn Fn(&mut Config, String)->(),
}

fn find_arg_def<'a>(arg:&str, argument_defs_accessors:&'a [ArgumentDefAccessor]) -> Option<&'a ArgumentDefAccessor> {
    for arg_def in argument_defs_accessors.iter() {
        if arg_def.argument_def.long_form == arg || arg_def.argument_def.short_form.unwrap() == arg {
            return Option::Some(arg_def);
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
        accessor: &Config::set_todo_filename,
    },
    ArgumentDefAccessor {
        argument_def: ArgumentDef {
            long_form: "--done-file",
            short_form: Option::Some("-d"),
            help_message: "done file",
            mandatory: true,
        },
        accessor: &Config::set_done_filename
    }
];


pub fn parse_arguments(args:&mut Args)->Result<Config, HashSet<&ArgumentDef>> {

    let mut config:Config = Config{ 
        todo_filename: Option::None,
        done_filename: Option::None,
    };

    let mut unset_arguments = HashSet::new();
    for arg_def_acc in ARGUMENT_DEFS_ACCESSORS.iter() {
        let argument_def: &ArgumentDef = &arg_def_acc.argument_def;
        if argument_def.mandatory {
            unset_arguments.insert(argument_def);
        }
    }
    
    while let Option::Some(arg) = args.next() {
        match find_arg_def(&arg, &ARGUMENT_DEFS_ACCESSORS) {
            Option::Some(arg_def) => {
                let argument = args.next();
                let message = format!("argument {} not present", &arg);
                (arg_def.accessor)(&mut config, argument.expect(&message));
                unset_arguments.remove(&arg_def.argument_def);
            },
            _ =>  ()
        }
    }
        
    if ! unset_arguments.is_empty() {
        Result::Err(unset_arguments)
    } else {
        Result::Ok(config)
    }
}