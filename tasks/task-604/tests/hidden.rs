use task_604::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_hash("unclosed[cat"), "");
    assert_eq!(first_nonempty_field_hash("[]"), "");
    assert_eq!(first_nonempty_field_hash("[42]"), "42");
    assert_eq!(first_nonempty_field_hash("[321]"), "321");
    assert_eq!(first_nonempty_field_hash("hi#kit"), "");
    assert_eq!(first_nonempty_field_hash("#é#a"), "");
    assert_eq!(first_nonempty_field_hash(""), "");
    assert_eq!(first_nonempty_field_hash("#"), "");
    assert_eq!(first_nonempty_field_hash("cat"), "");
    assert_eq!(first_nonempty_field_hash("##one#猫#ab"), "");
    assert_eq!(first_nonempty_field_hash("aa#bb"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_604::TRACK_PROOF, 0x52540000 | 604);
}
