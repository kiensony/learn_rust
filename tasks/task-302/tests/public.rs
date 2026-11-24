use task_302::*;

#[test]
fn public_examples() {
    assert_eq!(boundary_trim_bang("cat"), "cat");
    assert_eq!(boundary_trim_bang("!hi!"), "hi");
}
