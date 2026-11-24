use task_300::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_prefix_bang("cat"), "cat");
    assert_eq!(remove_a_prefix_bang("!hi!"), "hi!");
}
