use task_299::*;

#[test]
fn public_examples() {
    assert_eq!(wrap_a_message_bang("cat"), "!cat!");
    assert_eq!(wrap_a_message_bang("!hi!"), "!!hi!!");
}
