use task_506::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_hash(("unclosed[cat", "x")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("[]", "z#w")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("[42]", "x")), 2);
    assert_eq!(longest_field_with_last_tie_hash(("[321]", "z#w")), 3);
    assert_eq!(longest_field_with_last_tie_hash(("hi#kit", "x")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("#é#a", "z#w")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("", "x")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("#", "z#w")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("cat", "x")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(longest_field_with_last_tie_hash(("aa#bb", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_506::TRACK_PROOF, 0x52540000 | 506);
}
