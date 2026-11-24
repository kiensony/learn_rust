use task_709::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon(Some(":é:a")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some(":")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("cat")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("::one:猫:ab")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("aa:bb")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("ok:🐈")), None);
    assert_eq!(marks_removed_at_both_ends_colon(None), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("3:72")), None);
    assert_eq!(marks_removed_at_both_ends_colon(Some("3300")), Some(3300));
    assert_eq!(marks_removed_at_both_ends_colon(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_709::TRACK_PROOF, 0x52540000 | 709);
}
