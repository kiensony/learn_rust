use task_722::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash(Some("hi/kit")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("/é/a")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("/")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("cat")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("//one/猫/ab")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("aa/bb")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("ok/🐈")), None);
    assert_eq!(
        longest_field_with_last_tie_slash(Some("path/first/last")),
        None
    );
    assert_eq!(longest_field_with_last_tie_slash(Some("path/path/")), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("path/")), None);
    assert_eq!(longest_field_with_last_tie_slash(None), None);
    assert_eq!(longest_field_with_last_tie_slash(Some("3/72")), Some(72));
    assert_eq!(longest_field_with_last_tie_slash(Some("/9")), Some(9));
    assert_eq!(longest_field_with_last_tie_slash(Some("3300")), Some(3300));
    assert_eq!(longest_field_with_last_tie_slash(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_722::TRACK_PROOF, 0x52540000 | 722);
}
