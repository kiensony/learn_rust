use task_901::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_hash("unclosed[cat"), 2);
    assert_eq!(longest_field_with_last_tie_hash("[]"), 2);
    assert_eq!(longest_field_with_last_tie_hash("[42]"), 4);
    assert_eq!(longest_field_with_last_tie_hash("[321]"), 5);
    assert_eq!(longest_field_with_last_tie_hash("hi#kit"), 2);
    assert_eq!(longest_field_with_last_tie_hash("#é#a"), 2);
    assert_eq!(longest_field_with_last_tie_hash(""), 2);
    assert_eq!(longest_field_with_last_tie_hash("#"), 2);
    assert_eq!(longest_field_with_last_tie_hash("cat"), 2);
    assert_eq!(longest_field_with_last_tie_hash("##one#猫#ab"), 2);
    assert_eq!(longest_field_with_last_tie_hash("aa#bb"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_901::TRACK_PROOF, 0x52540000 | 901);
}
