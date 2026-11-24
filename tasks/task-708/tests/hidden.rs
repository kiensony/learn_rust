use task_708::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon(Some(":é:a")), None);
    assert_eq!(closing_marks_removed_colon(Some("")), None);
    assert_eq!(closing_marks_removed_colon(Some(":")), None);
    assert_eq!(closing_marks_removed_colon(Some("cat")), None);
    assert_eq!(closing_marks_removed_colon(Some("::one:猫:ab")), None);
    assert_eq!(closing_marks_removed_colon(Some("aa:bb")), None);
    assert_eq!(closing_marks_removed_colon(Some("ok:🐈")), None);
    assert_eq!(closing_marks_removed_colon(None), None);
    assert_eq!(closing_marks_removed_colon(Some("3:72")), None);
    assert_eq!(closing_marks_removed_colon(Some(":9")), None);
    assert_eq!(closing_marks_removed_colon(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_708::TRACK_PROOF, 0x52540000 | 708);
}
