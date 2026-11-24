use task_2315::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_changes(&[]), vec![]);
    assert_eq!(paired_speedup_changes(&[4]), vec![]);
}
