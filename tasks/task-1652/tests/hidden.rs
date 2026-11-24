use task_1652::*;

#[test]
fn edge_cases() {
    assert_eq!(major_version_total("1.0.0 1.2.0 2.0.0"), 4);
    assert_eq!(major_version_total("1 3.1.8 bad 3.1.8"), 6);
    assert_eq!(major_version_total("0.255.3 1.0.9 256.1.1"), 1);
    assert_eq!(major_version_total("12.4.8 2.9.3 12.4.8"), 26);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1652::TRACK_PROOF, 0x52540000 | 1652);
}
