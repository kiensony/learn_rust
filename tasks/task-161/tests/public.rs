use task_161::*;

#[test]
fn public_examples() {
    assert_eq!(door(true), "locked");
    assert_eq!(door(false), "open");
}
