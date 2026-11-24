use task_470::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_slash(("hi/kit", "x")), 6);
    assert_eq!(leading_marks_removed_slash(("/é/a", "z/w")), 4);
    assert_eq!(leading_marks_removed_slash(("", "x")), 1);
    assert_eq!(leading_marks_removed_slash(("/", "z/w")), 3);
    assert_eq!(leading_marks_removed_slash(("cat", "x")), 3);
    assert_eq!(leading_marks_removed_slash(("//one/猫/ab", "z/w")), 10);
    assert_eq!(leading_marks_removed_slash(("aa/bb", "x")), 5);
    assert_eq!(leading_marks_removed_slash(("ok/🐈", "z/w")), 7);
    assert_eq!(leading_marks_removed_slash(("path/first/last", "x")), 10);
    assert_eq!(leading_marks_removed_slash(("path/path/", "z/w")), 5);
    assert_eq!(leading_marks_removed_slash(("path/", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_470::TRACK_PROOF, 0x52540000 | 470);
}
