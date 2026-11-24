use task_1098::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_hash(&String::from("[猫###a]")), 3);
    assert_eq!(first_nonempty_field_hash(&String::from("[]")), 0);
    assert_eq!(first_nonempty_field_hash(&"[42]"), 2);
    assert_eq!(first_nonempty_field_hash(&String::from("[321]")), 3);
    assert_eq!(first_nonempty_field_hash(&"hi#kit"), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("#é#a")), 0);
    assert_eq!(first_nonempty_field_hash(&""), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("#")), 0);
    assert_eq!(first_nonempty_field_hash(&"cat"), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("##one#猫#ab")), 0);
    assert_eq!(first_nonempty_field_hash(&"aa#bb"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1098::TRACK_PROOF, 0x52540000 | 1098);
}
