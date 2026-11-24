use task_1596::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_scaling_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        if cfg!(feature = "wide") {
            Some((0, 6))
        } else {
            Some((0, 3))
        }
    );
    assert_eq!(feature_scaling_threshold_crossing(&[], 0), None);
}
