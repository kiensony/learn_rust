use task_280::*;

#[test]
fn public_examples() {
    assert_eq!(remove_a_prefix_tilde("cat"), "cat");
    assert_eq!(remove_a_prefix_tilde("~hi~"), "hi~");
}
