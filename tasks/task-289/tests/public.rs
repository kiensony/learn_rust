use task_289::*;

#[test]
fn public_examples() {
    assert_eq!(wrap_a_message_plus("cat"), "+cat+");
    assert_eq!(wrap_a_message_plus("+hi+"), "++hi++");
}
