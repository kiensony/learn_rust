use task_2317::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_distinct(&[]), vec![]);
    assert_eq!(paired_speedup_distinct(&[4]), vec![]);
}
