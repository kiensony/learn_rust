use task_605::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_hash("unclosed[cat"), "");
    assert_eq!(longest_field_with_last_tie_hash("[]"), "");
    assert_eq!(longest_field_with_last_tie_hash("[42]"), "42");
    assert_eq!(longest_field_with_last_tie_hash("[321]"), "321");
    assert_eq!(longest_field_with_last_tie_hash("hi#kit"), "");
    assert_eq!(longest_field_with_last_tie_hash("#é#a"), "");
    assert_eq!(longest_field_with_last_tie_hash(""), "");
    assert_eq!(longest_field_with_last_tie_hash("#"), "");
    assert_eq!(longest_field_with_last_tie_hash("cat"), "");
    assert_eq!(longest_field_with_last_tie_hash("##one#猫#ab"), "");
    assert_eq!(longest_field_with_last_tie_hash("aa#bb"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_605::TRACK_PROOF, 0x52540000 | 605);
}
