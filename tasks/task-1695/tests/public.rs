use task_1695::*;

#[test]
fn public_examples() {
    assert_eq!(release_line_changes(""), vec![]);
    assert_eq!(release_line_changes("0.0.1"), vec![0u32]);
}
