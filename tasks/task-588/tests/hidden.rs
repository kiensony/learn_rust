use task_588::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_equals("hi=kit"), "kit");
    assert_eq!(closing_field_equals("=é=a"), "a");
    assert_eq!(closing_field_equals(""), "");
    assert_eq!(closing_field_equals("="), "");
    assert_eq!(closing_field_equals("cat"), "cat");
    assert_eq!(closing_field_equals("==one=猫=ab"), "ab");
    assert_eq!(closing_field_equals("aa=bb"), "bb");
    assert_eq!(closing_field_equals("ok=🐈"), "🐈");
    assert_eq!(closing_field_equals("aa=bb;meta=123"), "bb");
    assert_eq!(closing_field_equals(";ignore=all"), "");
    assert_eq!(closing_field_equals("a==b;"), "b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_588::TRACK_PROOF, 0x52540000 | 588);
}
