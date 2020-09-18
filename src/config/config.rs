#[derive(Debug)]
pub struct Config {
    pub todo_filename: Option<String>,
    pub done_filename: Option<String>,
}

pub fn read_configuration_from_filecontent(file_content: &str, result: &mut Config){
    file_content.lines().for_each(|l| {
        let line: &str = l;
        let split: Vec<&str> = line.split('=').collect();
        if split.len()==2 {
            if split[0] == "todo_filename" {
                result.todo_filename = Option::Some(split[1].to_owned());
            }
            if split[0] == "done_filename" {
                result.done_filename = Option::Some(split[1].to_owned());
            }
        }
    })
}

