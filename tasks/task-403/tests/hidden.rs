use task_403::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash(&String::from("unclosed[cat")),
        0
    );
    assert_eq!(remainder_after_first_mark_hash(&String::from("[]")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("[42]")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("[321]")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("hi#kit")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("#é#a")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("#")), 0);
    assert_eq!(remainder_after_first_mark_hash(&String::from("cat")), 0);
    assert_eq!(
        remainder_after_first_mark_hash(&String::from("##one#猫#ab")),
        0
    );
    assert_eq!(remainder_after_first_mark_hash(&String::from("aa#bb")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_403::TRACK_PROOF, 0x52540000 | 403);
}
