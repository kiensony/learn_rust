use task_751::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_hash(Some("[猫###a]")), None);
    assert_eq!(first_nonempty_field_hash(Some("unclosed[cat")), None);
    assert_eq!(first_nonempty_field_hash(Some("[]")), None);
    assert_eq!(first_nonempty_field_hash(Some("[321]")), Some(321));
    assert_eq!(first_nonempty_field_hash(Some("hi#kit")), None);
    assert_eq!(first_nonempty_field_hash(Some("#é#a")), None);
    assert_eq!(first_nonempty_field_hash(Some("")), None);
    assert_eq!(first_nonempty_field_hash(Some("#")), None);
    assert_eq!(first_nonempty_field_hash(Some("cat")), None);
    assert_eq!(first_nonempty_field_hash(Some("##one#猫#ab")), None);
    assert_eq!(first_nonempty_field_hash(Some("aa#bb")), None);
    assert_eq!(first_nonempty_field_hash(Some("ok#🐈")), None);
    assert_eq!(first_nonempty_field_hash(None), None);
    assert_eq!(first_nonempty_field_hash(Some("3#72")), None);
    assert_eq!(first_nonempty_field_hash(Some("#9")), None);
    assert_eq!(first_nonempty_field_hash(Some("3300")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_751::TRACK_PROOF, 0x52540000 | 751);
}
