use task_1109::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_colon(&""), 0);
    assert_eq!(longest_field_with_last_tie_colon(&String::from(":")), 0);
    assert_eq!(longest_field_with_last_tie_colon(&"cat"), 3);
    assert_eq!(
        longest_field_with_last_tie_colon(&String::from("::one:猫:ab")),
        3
    );
    assert_eq!(longest_field_with_last_tie_colon(&"aa:bb"), 2);
    assert_eq!(longest_field_with_last_tie_colon(&String::from("ok:🐈")), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1109::TRACK_PROOF, 0x52540000 | 1109);
}
