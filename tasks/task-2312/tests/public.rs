use task_2312::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_total(&[]), 0);
    assert_eq!(paired_speedup_total(&[4]), 0);
}
