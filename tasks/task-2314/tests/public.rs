use task_2314::*;

#[test]
fn public_examples() {
    assert_eq!(paired_speedup_peaks(&[]), vec![]);
    assert_eq!(paired_speedup_peaks(&[4]), vec![]);
}
