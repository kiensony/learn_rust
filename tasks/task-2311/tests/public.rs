use task_2311::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_trace(&[]), vec![]);
    assert_eq!(paired_speedup_trace(&[4]), vec![]);
}
