use task_2235::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_changes(""), vec![]);
    assert_eq!(zero_copy_lines_changes("a"), vec![1u32]);
}
