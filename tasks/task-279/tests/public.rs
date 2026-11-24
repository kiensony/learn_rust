use task_279::*;

#[test]
fn public_examples() {
    assert_eq!(wrap_a_message_tilde("cat"), "~cat~");
    assert_eq!(wrap_a_message_tilde("~hi~"), "~~hi~~");
}
