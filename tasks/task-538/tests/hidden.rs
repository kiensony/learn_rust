use task_538::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals("hi=kit"), 2);
    assert_eq!(leading_field_equals("=é=a"), 0);
    assert_eq!(leading_field_equals(""), 0);
    assert_eq!(leading_field_equals("="), 0);
    assert_eq!(leading_field_equals("cat"), 3);
    assert_eq!(leading_field_equals("==one=猫=ab"), 0);
    assert_eq!(leading_field_equals("aa=bb"), 2);
    assert_eq!(leading_field_equals("ok=🐈"), 2);
    assert_eq!(leading_field_equals("aa=bb;meta=123"), 2);
    assert_eq!(leading_field_equals(";ignore=all"), 0);
    assert_eq!(leading_field_equals("a==b;"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_538::TRACK_PROOF, 0x52540000 | 538);
}
