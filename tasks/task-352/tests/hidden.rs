use task_352::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_hash(String::from("unclosed[cat")),
        ""
    );
    assert_eq!(marks_removed_at_both_ends_hash(String::from("[]")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("[42]")), "42");
    assert_eq!(
        marks_removed_at_both_ends_hash(String::from("[321]")),
        "321"
    );
    assert_eq!(marks_removed_at_both_ends_hash(String::from("hi#kit")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("#é#a")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("#")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("cat")), "");
    assert_eq!(
        marks_removed_at_both_ends_hash(String::from("##one#猫#ab")),
        ""
    );
    assert_eq!(marks_removed_at_both_ends_hash(String::from("aa#bb")), "");
    assert_eq!(marks_removed_at_both_ends_hash(String::from("ok#🐈")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_352::TRACK_PROOF, 0x52540000 | 352);
}
