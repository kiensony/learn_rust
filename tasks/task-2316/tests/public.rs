use task_2316::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_descending_steps(&[]), 0);
    assert_eq!(paired_speedup_descending_steps(&[4]), 0);
}
