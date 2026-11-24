use task_500::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_hash(("unclosed[cat", "x")), 0);
    assert_eq!(leading_marks_removed_hash(("[]", "z#w")), 0);
    assert_eq!(leading_marks_removed_hash(("[42]", "x")), 2);
    assert_eq!(leading_marks_removed_hash(("[321]", "z#w")), 3);
    assert_eq!(leading_marks_removed_hash(("hi#kit", "x")), 0);
    assert_eq!(leading_marks_removed_hash(("#é#a", "z#w")), 0);
    assert_eq!(leading_marks_removed_hash(("", "x")), 0);
    assert_eq!(leading_marks_removed_hash(("#", "z#w")), 0);
    assert_eq!(leading_marks_removed_hash(("cat", "x")), 0);
    assert_eq!(leading_marks_removed_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(leading_marks_removed_hash(("aa#bb", "x")), 0);
    assert_eq!(leading_marks_removed_hash(("ok#🐈", "z#w")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_500::TRACK_PROOF, 0x52540000 | 500);
}
