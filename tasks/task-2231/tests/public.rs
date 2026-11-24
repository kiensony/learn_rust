use task_2231::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_trace(""), vec![]);
    assert_eq!(zero_copy_lines_trace("a"), vec![1u32]);
}
