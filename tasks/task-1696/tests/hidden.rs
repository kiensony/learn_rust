use task_1696::*;

#[test]
fn edge_cases() {
    assert_eq!(major_version_threshold_crossing("0.0.1", 0), Some((0, 0)));
    assert_eq!(
        major_version_threshold_crossing("1.0.0 1.2.0 2.0.0", 1),
        Some((0, 1))
    );
    assert_eq!(
        major_version_threshold_crossing("1 3.1.8 bad 3.1.8", 3),
        Some((0, 3))
    );
    assert_eq!(
        major_version_threshold_crossing("0.255.3 1.0.9 256.1.1", 256),
        None
    );
    assert_eq!(
        major_version_threshold_crossing("12.4.8 2.9.3 12.4.8", 0),
        Some((0, 12))
    );
    assert_eq!(
        major_version_threshold_crossing("1.0.0 1.2.0 2.0.0", 3),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1696::TRACK_PROOF, 0x52540000 | 1696);
}
