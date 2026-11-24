use task_495::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_equals(("hi=kit", "x")), 2);
    assert_eq!(first_nonempty_field_equals(("=é=a", "z=w")), 2);
    assert_eq!(first_nonempty_field_equals(("", "x")), 1);
    assert_eq!(first_nonempty_field_equals(("=", "z=w")), 1);
    assert_eq!(first_nonempty_field_equals(("cat", "x")), 3);
    assert_eq!(first_nonempty_field_equals(("==one=猫=ab", "z=w")), 3);
    assert_eq!(first_nonempty_field_equals(("aa=bb", "x")), 2);
    assert_eq!(first_nonempty_field_equals(("ok=🐈", "z=w")), 2);
    assert_eq!(first_nonempty_field_equals(("aa=bb;meta=123", "x")), 2);
    assert_eq!(first_nonempty_field_equals((";ignore=all", "z=w")), 0);
    assert_eq!(first_nonempty_field_equals(("a==b;", "x")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_495::TRACK_PROOF, 0x52540000 | 495);
}
