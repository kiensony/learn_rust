use task_1686::*;

#[test]
fn edge_cases() {
    assert_eq!(preview_version_descending_steps("1.0.0 1.2.0 2.0.0"), 0);
    assert_eq!(preview_version_descending_steps("1 3.1.8 bad 3.1.8"), 0);
    assert_eq!(preview_version_descending_steps("0.255.3 1.0.9 256.1.1"), 1);
    assert_eq!(preview_version_descending_steps("12.4.8 2.9.3 12.4.8"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1686::TRACK_PROOF, 0x52540000 | 1686);
}
