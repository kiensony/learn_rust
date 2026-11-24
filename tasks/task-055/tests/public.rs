mod common;

#[test]
fn examples() {
    common::check(&["101"], "R-101\n", "");
    common::check(&["007"], "R-007\n", "");
}

#[test]
fn reusable_label() {
    assert_eq!(task_055::receipt("101"), "R-101");
    assert_eq!(task_055::receipt("007"), "R-007");
}
