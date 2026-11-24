mod common;

#[test]
fn examples() {
    common::check(&[], "package task_032\n", "");
    common::check(&["unused"], "package task_032\n", "");
}
