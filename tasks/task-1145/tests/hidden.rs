use task_1145::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_hash(&"unclosed[cat"), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&String::from("[]")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&"[42]"), 2);
    assert_eq!(marks_removed_at_both_ends_hash(&String::from("[321]")), 3);
    assert_eq!(marks_removed_at_both_ends_hash(&"hi#kit"), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&String::from("#é#a")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&""), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&String::from("#")), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&"cat"), 0);
    assert_eq!(
        marks_removed_at_both_ends_hash(&String::from("##one#猫#ab")),
        0
    );
    assert_eq!(marks_removed_at_both_ends_hash(&"aa#bb"), 0);
    assert_eq!(marks_removed_at_both_ends_hash(&String::from("ok#🐈")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1145::TRACK_PROOF, 0x52540000 | 1145);
}
