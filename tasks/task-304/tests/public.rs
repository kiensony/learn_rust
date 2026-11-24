use task_304::*;

#[test]
fn public_examples() {
    assert_eq!(read_a_heading_bang("cat"), "cat");
    assert_eq!(read_a_heading_bang("!hi!"), "");
}
