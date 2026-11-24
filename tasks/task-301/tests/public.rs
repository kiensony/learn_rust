use task_301::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_suffix_bang("cat"), "cat");
    assert_eq!(remove_a_suffix_bang("!hi!"), "!hi");
}
