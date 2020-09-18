#[test]
pub fn test_parse_config() {
    use crate::*;
    let mut config = crate::config::Config {
        todo_filename: Option::None,
        done_filename: Option::None,
    };
    read_configuration_from_filecontent(r#"
todo_filename=/home/avd/todo.txt
done_filename=/home/avd/done.txt
    "#, &mut config);
    println!("{:?}", &config);
    assert!(matches!(config, Config { todo_filename: Option::Some(v), ..}))
}
