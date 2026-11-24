use task_2232::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_total(""), 0);
    assert_eq!(zero_copy_lines_total("a"), 1);
}
