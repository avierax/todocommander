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

#[test]
pub fn test_add() {
    let mut model = create_model_for_testing();
    model
        .execute(Command::Add("do this at @Office".into()))
        .expect("test failed");
    assert_eq!(
        model,
        Model {
            todo_data: TodoData::parse(
                "do something +home
do something else +work
do this at @Office"
            )
            .unwrap(),
            done_data: TodoData { entries: vec![] }
        },
        "Models are not equal"
    );
}

#[test]
pub fn test_do() {
    let mut model = create_model_for_testing();
    model.execute(Command::Do(0)).expect("test failed");
    let Model {
        todo_data: TodoData { entries },
        ..
    } = model;
    assert!(matches!(entries[0].status, Status::Done(_)));
}

#[test]
pub fn test_do_undo() {
    let mut model = create_model_for_testing();
    model.execute(Command::Do(0)).expect("test failed");
    model.execute(Command::Undo(0)).expect("test failed");
    let Model {
        todo_data: TodoData { entries },
        ..
    } = model;
    assert!(matches!(entries[0].status, Status::Open));
}
