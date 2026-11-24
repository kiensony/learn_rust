use task_066::*;

#[test]
fn edge_cases() {
    assert_eq!(bundle_labels_2(0), 0);
    assert_eq!(bundle_labels_2(1), 3);
    assert_eq!(bundle_labels_2(1), 3);
    assert_eq!(bundle_labels_2(2), 6);
    assert_eq!(bundle_labels_2(3), 9);
    assert_eq!(bundle_labels_2(65535), 196605);
}

#[test]
fn submission_marker() {
    assert_eq!(task_066::TRACK_PROOF, 0x52540000 | 66);
}
