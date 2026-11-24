use task_1662::*;

#[test]
fn edge_cases() {
    assert_eq!(minor_version_total("1.0.0 1.2.0 2.0.0"), 2);
    assert_eq!(minor_version_total("1 3.1.8 bad 3.1.8"), 2);
    assert_eq!(minor_version_total("0.255.3 1.0.9 256.1.1"), 255);
    assert_eq!(minor_version_total("12.4.8 2.9.3 12.4.8"), 17);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1662::TRACK_PROOF, 0x52540000 | 1662);
}
