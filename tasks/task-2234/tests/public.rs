use task_2234::*;

#[test]
fn public_examples() {
    assert_eq!(zero_copy_lines_peaks(""), vec![]);
    assert_eq!(zero_copy_lines_peaks("a"), vec![1u32]);
}
