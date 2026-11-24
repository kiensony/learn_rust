use task_2313::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_bounds(&[]), None);
    assert_eq!(paired_speedup_bounds(&[4]), None);
}
