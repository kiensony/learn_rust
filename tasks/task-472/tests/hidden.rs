use task_472::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash(("hi/kit", "x")), 6);
    assert_eq!(marks_removed_at_both_ends_slash(("/é/a", "z/w")), 4);
    assert_eq!(marks_removed_at_both_ends_slash(("", "x")), 1);
    assert_eq!(marks_removed_at_both_ends_slash(("/", "z/w")), 3);
    assert_eq!(marks_removed_at_both_ends_slash(("cat", "x")), 3);
    assert_eq!(marks_removed_at_both_ends_slash(("//one/猫/ab", "z/w")), 10);
    assert_eq!(marks_removed_at_both_ends_slash(("aa/bb", "x")), 5);
    assert_eq!(marks_removed_at_both_ends_slash(("ok/🐈", "z/w")), 7);
    assert_eq!(
        marks_removed_at_both_ends_slash(("path/first/last", "x")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_slash(("path/path/", "z/w")), 4);
    assert_eq!(marks_removed_at_both_ends_slash(("path/", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_472::TRACK_PROOF, 0x52540000 | 472);
}
