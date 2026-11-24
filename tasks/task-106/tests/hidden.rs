use task_106::*;

#[test]
fn edge_cases() {
    assert_eq!(bundle_labels_10(65535, 0), 0);
    assert_eq!(bundle_labels_10(1, 2), 11);
    assert_eq!(bundle_labels_10(65535, 9), 99);
    assert_eq!(bundle_labels_10(10, 11), 110);
    assert_eq!(bundle_labels_10(65535, 11), 121);
    assert_eq!(bundle_labels_10(65535, 65535), 720885);
    assert_eq!(bundle_labels_10(1, 0), 0);
    assert_eq!(bundle_labels_10(0, 1), 0);
    assert_eq!(bundle_labels_10(0, 0), 0);
    assert_eq!(bundle_labels_10(1, 1), 11);
    assert_eq!(bundle_labels_10(9, 9), 99);
    assert_eq!(bundle_labels_10(10, 10), 110);
    assert_eq!(bundle_labels_10(11, 11), 121);
    assert_eq!(bundle_labels_10(12, 12), 132);
    assert_eq!(bundle_labels_10(13, 13), 143);
    assert_eq!(bundle_labels_10(20, 20), 220);
    assert_eq!(bundle_labels_10(21, 21), 231);
}

#[test]
fn submission_marker() {
    assert_eq!(task_106::TRACK_PROOF, 0x52540000 | 106);
}
