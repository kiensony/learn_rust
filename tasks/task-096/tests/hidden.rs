use task_096::*;

#[test]
fn edge_cases() {
    assert_eq!(bundle_labels_8(0, 2), 18);
    assert_eq!(bundle_labels_8(1, 3), 18);
    assert_eq!(bundle_labels_8(7, 4), 27);
    assert_eq!(bundle_labels_8(8, 5), 27);
    assert_eq!(bundle_labels_8(9, 6), 27);
    assert_eq!(bundle_labels_8(65535, 0), 589815);
    assert_eq!(bundle_labels_8(18, 3), 135);
    assert_eq!(bundle_labels_8(3, 18), 135);
    assert_eq!(bundle_labels_8(4, 4), 0);
    assert_eq!(bundle_labels_8(0, 0), 0);
    assert_eq!(bundle_labels_8(1, 0), 9);
    assert_eq!(bundle_labels_8(7, 0), 63);
    assert_eq!(bundle_labels_8(8, 0), 72);
    assert_eq!(bundle_labels_8(9, 0), 81);
    assert_eq!(bundle_labels_8(10, 0), 90);
    assert_eq!(bundle_labels_8(11, 0), 99);
    assert_eq!(bundle_labels_8(16, 0), 144);
    assert_eq!(bundle_labels_8(17, 0), 153);
    assert_eq!(bundle_labels_8(20, 0), 180);
    assert_eq!(bundle_labels_8(100, 0), 900);
}

#[test]
fn submission_marker() {
    assert_eq!(task_096::TRACK_PROOF, 0x52540000 | 96);
}
