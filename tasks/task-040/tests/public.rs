mod common;

#[test]
fn examples() {
    common::check(&[], "desk ready\nline ready\n", "");
    common::check(&["unused"], "desk ready\nline ready\n", "");
}

#[test]
fn reusable_value() {
    assert_eq!(task_040::STATUS, "ready");
}
