use task_2320::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_reverse(&[]), vec![]);
    assert_eq!(paired_speedup_reverse(&[4]), vec![]);
}
