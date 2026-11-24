use task_748::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash(Some("[猫###a]")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("unclosed[cat")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("[]")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("[321]")), Some(321));
    assert_eq!(marks_removed_at_both_ends_hash(Some("hi#kit")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("#é#a")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("#")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("cat")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("##one#猫#ab")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("aa#bb")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("ok#🐈")), None);
    assert_eq!(marks_removed_at_both_ends_hash(None), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("3#72")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("#9")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("3300")), None);
    assert_eq!(marks_removed_at_both_ends_hash(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_748::TRACK_PROOF, 0x52540000 | 748);
}
