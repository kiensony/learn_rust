use task_353::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash(String::from("unclosed[cat")),
        ""
    );
    assert_eq!(remainder_after_first_mark_hash(String::from("[]")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("[42]")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("[321]")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("hi#kit")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("#é#a")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("#")), "");
    assert_eq!(remainder_after_first_mark_hash(String::from("cat")), "");
    assert_eq!(
        remainder_after_first_mark_hash(String::from("##one#猫#ab")),
        ""
    );
    assert_eq!(remainder_after_first_mark_hash(String::from("aa#bb")), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_353::TRACK_PROOF, 0x52540000 | 353);
}
