use task_587::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_field_equals("hi=kit"), "hi");
    assert_eq!(leading_field_equals("=é=a"), "");
    assert_eq!(leading_field_equals(""), "");
    assert_eq!(leading_field_equals("="), "");
    assert_eq!(leading_field_equals("cat"), "cat");
    assert_eq!(leading_field_equals("==one=猫=ab"), "");
    assert_eq!(leading_field_equals("aa=bb"), "aa");
    assert_eq!(leading_field_equals("ok=🐈"), "ok");
    assert_eq!(leading_field_equals("aa=bb;meta=123"), "aa");
    assert_eq!(leading_field_equals(";ignore=all"), "");
    assert_eq!(leading_field_equals("a==b;"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_587::TRACK_PROOF, 0x52540000 | 587);
}
