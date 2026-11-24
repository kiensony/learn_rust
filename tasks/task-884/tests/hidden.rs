use task_884::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals("猫=12;note"), 4);
    assert_eq!(closing_field_equals("=é=a"), 3);
    assert_eq!(closing_field_equals(""), 2);
    assert_eq!(closing_field_equals("="), 2);
    assert_eq!(closing_field_equals("cat"), 5);
    assert_eq!(closing_field_equals("==one=猫=ab"), 4);
    assert_eq!(closing_field_equals("aa=bb"), 4);
    assert_eq!(closing_field_equals("ok=🐈"), 6);
    assert_eq!(closing_field_equals("aa=bb;meta=123"), 4);
    assert_eq!(closing_field_equals(";ignore=all"), 2);
    assert_eq!(closing_field_equals("a==b;"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_884::TRACK_PROOF, 0x52540000 | 884);
}
