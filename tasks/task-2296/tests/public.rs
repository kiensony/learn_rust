use task_2296::*;

#[test]
fn public_examples() {
    assert_eq!(
        cpu_sample_counts_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 1))
    );
    assert_eq!(cpu_sample_counts_threshold_crossing(&[], 0), None);
}
