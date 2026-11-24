use task_356::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_hash(String::from("unclosed[cat")),
        ""
    );
    assert_eq!(longest_field_with_last_tie_hash(String::from("[]")), "");
    assert_eq!(longest_field_with_last_tie_hash(String::from("[42]")), "42");
    assert_eq!(
        longest_field_with_last_tie_hash(String::from("[321]")),
        "321"
    );
    assert_eq!(longest_field_with_last_tie_hash(String::from("hi#kit")), "");
    assert_eq!(longest_field_with_last_tie_hash(String::from("#é#a")), "");
    assert_eq!(longest_field_with_last_tie_hash(String::from("")), "");
    assert_eq!(longest_field_with_last_tie_hash(String::from("#")), "");
    assert_eq!(longest_field_with_last_tie_hash(String::from("cat")), "");
    assert_eq!(
        longest_field_with_last_tie_hash(String::from("##one#猫#ab")),
        ""
    );
    assert_eq!(longest_field_with_last_tie_hash(String::from("aa#bb")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_356::TRACK_PROOF, 0x52540000 | 356);
}
