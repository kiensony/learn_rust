use task_1996::*;

#[test]
fn public_examples() {
    assert_eq!(
        atomic_running_total_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 0))
    );
    assert_eq!(atomic_running_total_threshold_crossing(&[], 0), None);
}
