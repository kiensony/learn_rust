mod common;

#[test]
fn examples() {
    common::check(&[], "task_058/1.3.0\n", "");
    common::check(&["unused"], "task_058/1.3.0\n", "");
}
