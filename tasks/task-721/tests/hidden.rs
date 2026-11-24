use task_721::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_slash(Some("hi/kit")), None);
    assert_eq!(first_nonempty_field_slash(Some("/é/a")), None);
    assert_eq!(first_nonempty_field_slash(Some("")), None);
    assert_eq!(first_nonempty_field_slash(Some("/")), None);
    assert_eq!(first_nonempty_field_slash(Some("cat")), None);
    assert_eq!(first_nonempty_field_slash(Some("//one/猫/ab")), None);
    assert_eq!(first_nonempty_field_slash(Some("aa/bb")), None);
    assert_eq!(first_nonempty_field_slash(Some("ok/🐈")), None);
    assert_eq!(first_nonempty_field_slash(Some("path/first/last")), None);
    assert_eq!(first_nonempty_field_slash(Some("path/path/")), None);
    assert_eq!(first_nonempty_field_slash(Some("path/")), None);
    assert_eq!(first_nonempty_field_slash(None), None);
    assert_eq!(first_nonempty_field_slash(Some("3/72")), Some(3));
    assert_eq!(first_nonempty_field_slash(Some("/9")), Some(9));
    assert_eq!(first_nonempty_field_slash(Some("3300")), Some(3300));
    assert_eq!(first_nonempty_field_slash(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_721::TRACK_PROOF, 0x52540000 | 721);
}
