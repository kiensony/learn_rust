use task_288::*;

#[test]
fn public_examples() {
    assert_eq!(normalize_spaces_tilde("cat"), "cat");
    assert_eq!(normalize_spaces_tilde("~hi~"), "~hi~");
}
