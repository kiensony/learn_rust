use task_2318::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_steps(&[]), vec![]);
    assert_eq!(paired_speedup_steps(&[4]), vec![]);
}
