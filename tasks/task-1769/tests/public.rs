use task_1769::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_longest_run(&[]), 0);
    assert_eq!(generated_offset_longest_run(&[4]), 1);
}
