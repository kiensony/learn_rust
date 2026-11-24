use task_274::*;

#[test]
fn public_examples() {
    assert_eq!(read_a_heading_star("cat"), "cat");
    assert_eq!(read_a_heading_star("*hi*"), "");
}
