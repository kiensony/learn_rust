use task_086::*;

#[test]
fn edge_cases() {
    assert_eq!(bundle_labels_5(0, 2), 0);
    assert_eq!(bundle_labels_5(1, 0), 6);
    assert_eq!(bundle_labels_5(4, 1), 18);
    assert_eq!(bundle_labels_5(5, 2), 18);
    assert_eq!(bundle_labels_5(6, 0), 36);
    assert_eq!(bundle_labels_5(65535, 1), 393204);
    assert_eq!(bundle_labels_5(0, 9), 0);
    assert_eq!(bundle_labels_5(100, 0), 600);
    assert_eq!(bundle_labels_5(100, 1), 594);
    assert_eq!(bundle_labels_5(0, 0), 0);
    assert_eq!(bundle_labels_5(5, 0), 30);
    assert_eq!(bundle_labels_5(7, 0), 42);
    assert_eq!(bundle_labels_5(8, 0), 48);
    assert_eq!(bundle_labels_5(10, 0), 60);
    assert_eq!(bundle_labels_5(11, 0), 66);
    assert_eq!(bundle_labels_5(20, 0), 120);
}

#[test]
fn submission_marker() {
    assert_eq!(task_086::TRACK_PROOF, 0x52540000 | 86);
}
