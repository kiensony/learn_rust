use task_306::*;

#[test]
fn public_examples() {
    assert_eq!(blank_notice_bang("cat"), "cat");
    assert_eq!(blank_notice_bang("!hi!"), "!hi!");
}
