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


struct ArgumentDef {
    long_form: &'static str,
    short_form: Option<&'static str>,
    help_message: &'static str,
    mandatory: bool,
    accessor: &'static dyn Fn(&mut Config, String)->(),
}

fn find_arg_def<'a>(arg:&str, argument_defs:&'a [ArgumentDef;2]) -> Option<&'a ArgumentDef> {
    for arg_def in argument_defs.iter() {
        if arg_def.long_form == arg || arg_def.short_form.unwrap() == arg {
            return Option::Some(arg_def);
        }
    }
    Option::None
}

pub fn parse_arguments(args:&mut Args)->Config {
    let argument_defs:[ArgumentDef;2] = [
        ArgumentDef{
            long_form: "--todo-file",
            short_form: Option::Some("-f"),
            help_message: "todo file",
            mandatory: true,
            accessor: &Config::set_todo_filename,
        },
        ArgumentDef{
            long_form: "--done-file",
            short_form: Option::Some("-d"),
            help_message: "done file",
            mandatory: true,
            accessor: &Config::set_done_filename
        }
    ];

    let mut config:Config = Config{ 
        todo_filename: Option::None,
        done_filename: Option::None,
    };
    
    while let Option::Some(arg) = args.next() {
        match find_arg_def(&arg, &argument_defs) {
            Option::Some(arg_def) => {
                let argument = args.next();
                let message = format!("argument {} not present", &arg);
                (arg_def.accessor)(&mut config, argument.expect(&message));

            },
            _ =>  ()
        }
    }

    todo!();

}