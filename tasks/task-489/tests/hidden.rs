use task_489::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals(("猫=12;note", "z=w")), 2);
    assert_eq!(closing_field_equals(("=é=a", "z=w")), 1);
    assert_eq!(closing_field_equals(("", "x")), 1);
    assert_eq!(closing_field_equals(("=", "z=w")), 1);
    assert_eq!(closing_field_equals(("cat", "x")), 3);
    assert_eq!(closing_field_equals(("==one=猫=ab", "z=w")), 2);
    assert_eq!(closing_field_equals(("aa=bb", "x")), 2);
    assert_eq!(closing_field_equals(("ok=🐈", "z=w")), 4);
    assert_eq!(closing_field_equals(("aa=bb;meta=123", "x")), 2);
    assert_eq!(closing_field_equals((";ignore=all", "z=w")), 0);
    assert_eq!(closing_field_equals(("a==b;", "x")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_489::TRACK_PROOF, 0x52540000 | 489);
}
