use task_1446::*;

#[test]
fn public_examples() {
    assert_eq!(
        millisecond_remainder_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 51))
    );
    assert_eq!(millisecond_remainder_threshold_crossing(&[], 0), None);
}
