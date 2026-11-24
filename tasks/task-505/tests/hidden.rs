use task_505::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_hash(("[猫###a]", "z#w")), 3);
    assert_eq!(first_nonempty_field_hash(("[]", "z#w")), 0);
    assert_eq!(first_nonempty_field_hash(("[42]", "x")), 2);
    assert_eq!(first_nonempty_field_hash(("[321]", "z#w")), 3);
    assert_eq!(first_nonempty_field_hash(("hi#kit", "x")), 0);
    assert_eq!(first_nonempty_field_hash(("#é#a", "z#w")), 0);
    assert_eq!(first_nonempty_field_hash(("", "x")), 0);
    assert_eq!(first_nonempty_field_hash(("#", "z#w")), 0);
    assert_eq!(first_nonempty_field_hash(("cat", "x")), 0);
    assert_eq!(first_nonempty_field_hash(("##one#猫#ab", "z#w")), 0);
    assert_eq!(first_nonempty_field_hash(("aa#bb", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_505::TRACK_PROOF, 0x52540000 | 505);
}
