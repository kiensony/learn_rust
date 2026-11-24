use task_712::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_colon(Some(":é:a")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some(":")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("cat")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("::one:猫:ab")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("aa:bb")), None);
    assert_eq!(longest_field_with_last_tie_colon(Some("ok:🐈")), None);
    assert_eq!(longest_field_with_last_tie_colon(None), None);
    assert_eq!(longest_field_with_last_tie_colon(Some(":9")), Some(9));
    assert_eq!(longest_field_with_last_tie_colon(Some("3300")), Some(3300));
    assert_eq!(longest_field_with_last_tie_colon(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_712::TRACK_PROOF, 0x52540000 | 712);
}
