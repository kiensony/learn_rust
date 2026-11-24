use task_2237::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_distinct(""), vec![]);
    assert_eq!(zero_copy_lines_distinct("a"), vec![1u32]);
}
