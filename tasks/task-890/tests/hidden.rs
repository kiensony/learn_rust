use task_890::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_equals("hi=kit"), 4);
    assert_eq!(first_nonempty_field_equals("=é=a"), 4);
    assert_eq!(first_nonempty_field_equals(""), 2);
    assert_eq!(first_nonempty_field_equals("="), 2);
    assert_eq!(first_nonempty_field_equals("cat"), 5);
    assert_eq!(first_nonempty_field_equals("==one=猫=ab"), 5);
    assert_eq!(first_nonempty_field_equals("aa=bb"), 4);
    assert_eq!(first_nonempty_field_equals("ok=🐈"), 4);
    assert_eq!(first_nonempty_field_equals("aa=bb;meta=123"), 4);
    assert_eq!(first_nonempty_field_equals(";ignore=all"), 2);
    assert_eq!(first_nonempty_field_equals("a==b;"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_890::TRACK_PROOF, 0x52540000 | 890);
}
