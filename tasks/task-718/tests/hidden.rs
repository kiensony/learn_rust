use task_718::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash(Some("hi/kit")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("/é/a")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("/")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("cat")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("//one/猫/ab")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("aa/bb")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("ok/🐈")), None);
    assert_eq!(
        marks_removed_at_both_ends_slash(Some("path/first/last")),
        None
    );
    assert_eq!(marks_removed_at_both_ends_slash(Some("path/path/")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("path/")), None);
    assert_eq!(marks_removed_at_both_ends_slash(None), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("3/72")), None);
    assert_eq!(marks_removed_at_both_ends_slash(Some("/9")), Some(9));
    assert_eq!(marks_removed_at_both_ends_slash(Some("3300")), Some(3300));
    assert_eq!(marks_removed_at_both_ends_slash(Some("65536")), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_718::TRACK_PROOF, 0x52540000 | 718);
}
