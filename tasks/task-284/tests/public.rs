use task_284::*;

#[test]
fn public_examples() {
    assert_eq!(read_a_heading_tilde("cat"), "cat");
    assert_eq!(read_a_heading_tilde("~hi~"), "");
}
