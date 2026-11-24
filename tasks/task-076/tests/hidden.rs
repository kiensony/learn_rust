use task_076::*;

#[test]
fn edge_cases() {
    assert_eq!(bundle_labels_3(0, 6), 24);
    assert_eq!(bundle_labels_3(1, 9), 40);
    assert_eq!(bundle_labels_3(2, 12), 56);
    assert_eq!(bundle_labels_3(3, 15), 72);
    assert_eq!(bundle_labels_3(4, 1), 20);
    assert_eq!(bundle_labels_3(65535, 4), 262140);
    assert_eq!(bundle_labels_3(65535, 1), 262140);
    assert_eq!(bundle_labels_3(0, 0), 0);
    assert_eq!(bundle_labels_3(1, 0), 4);
    assert_eq!(bundle_labels_3(2, 0), 8);
    assert_eq!(bundle_labels_3(3, 0), 12);
    assert_eq!(bundle_labels_3(5, 0), 20);
    assert_eq!(bundle_labels_3(6, 0), 24);
    assert_eq!(bundle_labels_3(7, 0), 28);
    assert_eq!(bundle_labels_3(20, 0), 80);
    assert_eq!(bundle_labels_3(100, 0), 400);
}

#[test]
fn submission_marker() {
    assert_eq!(task_076::TRACK_PROOF, 0x52540000 | 76);
}
