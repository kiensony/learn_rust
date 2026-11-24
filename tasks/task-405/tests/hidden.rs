use task_405::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_hash(&String::from("[猫###a]")), 3);
    assert_eq!(first_nonempty_field_hash(&String::from("[]")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("[42]")), 2);
    assert_eq!(first_nonempty_field_hash(&String::from("[321]")), 3);
    assert_eq!(first_nonempty_field_hash(&String::from("hi#kit")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("#é#a")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("#")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("cat")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("##one#猫#ab")), 0);
    assert_eq!(first_nonempty_field_hash(&String::from("aa#bb")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_405::TRACK_PROOF, 0x52540000 | 405);
}
