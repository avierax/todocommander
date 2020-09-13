/*
 * Model operation tests
 */

use crate::model::*;

fn create_model_for_testing() -> Model {
    Model {
        todo_data: TodoData::parse(
            "do something +home
do something else +work",
        )
        .unwrap(),
        done_data: TodoData { entries: vec![] },
    }
}

#[test]
pub fn test_archive() {
    let mut model = create_model_for_testing();
    match model.execute(Command::Archive(0)) {
        Result::Ok(_) => assert_eq!(
            model,
            Model {
                todo_data: TodoData::parse("do something else +work").unwrap(),
                done_data: TodoData::parse("do something +home").unwrap()
            },
            "Models are not equal"
        ),
        Result::Err(message) => panic!("This test failed. Reason: {}", message),
    }
}
