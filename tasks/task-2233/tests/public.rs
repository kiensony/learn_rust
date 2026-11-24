use task_2233::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_bounds(""), None);
    assert_eq!(zero_copy_lines_bounds("a"), Some((1u32, 1u32)));
}
