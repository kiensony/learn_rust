use task_1149::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_hash(&"unclosed[cat"), 0);
    assert_eq!(longest_field_with_last_tie_hash(&String::from("[]")), 0);
    assert_eq!(longest_field_with_last_tie_hash(&"[42]"), 2);
    assert_eq!(longest_field_with_last_tie_hash(&String::from("[321]")), 3);
    assert_eq!(longest_field_with_last_tie_hash(&"hi#kit"), 0);
    assert_eq!(longest_field_with_last_tie_hash(&String::from("#é#a")), 0);
    assert_eq!(longest_field_with_last_tie_hash(&""), 0);
    assert_eq!(longest_field_with_last_tie_hash(&String::from("#")), 0);
    assert_eq!(longest_field_with_last_tie_hash(&"cat"), 0);
    assert_eq!(
        longest_field_with_last_tie_hash(&String::from("##one#猫#ab")),
        0
    );
    assert_eq!(longest_field_with_last_tie_hash(&"aa#bb"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1149::TRACK_PROOF, 0x52540000 | 1149);
}
