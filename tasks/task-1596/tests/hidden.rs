use task_1596::*;

#[test]
fn edge_cases() {
    assert_eq!(
        feature_scaling_threshold_crossing(&[4], 0),
        if cfg!(feature = "wide") {
            Some((0, 8))
        } else {
            Some((0, 4))
        }
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[9, 2, 2, 8, 0, 5], 1),
        if cfg!(feature = "wide") {
            Some((0, 18))
        } else {
            Some((0, 9))
        }
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[0, 0, 0], 0),
        Some((0, 0))
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[255, 1, 255, 2], 256),
        if cfg!(feature = "wide") {
            Some((0, 510))
        } else {
            None
        }
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        if cfg!(feature = "wide") {
            Some((0, 6))
        } else {
            Some((0, 3))
        }
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[17, 17, 1, 0, 2], 1),
        if cfg!(feature = "wide") {
            Some((0, 34))
        } else {
            Some((0, 17))
        }
    );
    assert_eq!(
        feature_scaling_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 32),
        if cfg!(feature = "wide") {
            Some((4, 42))
        } else {
            None
        }
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1596::TRACK_PROOF, 0x52540000 | 1596);
}
