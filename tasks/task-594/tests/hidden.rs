use task_594::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_equals("hi=kit"), "hi");
    assert_eq!(first_nonempty_field_equals("=é=a"), "é");
    assert_eq!(first_nonempty_field_equals(""), "");
    assert_eq!(first_nonempty_field_equals("="), "");
    assert_eq!(first_nonempty_field_equals("cat"), "cat");
    assert_eq!(first_nonempty_field_equals("==one=猫=ab"), "one");
    assert_eq!(first_nonempty_field_equals("aa=bb"), "aa");
    assert_eq!(first_nonempty_field_equals("ok=🐈"), "ok");
    assert_eq!(first_nonempty_field_equals("aa=bb;meta=123"), "aa");
    assert_eq!(first_nonempty_field_equals(";ignore=all"), "");
    assert_eq!(first_nonempty_field_equals("a==b;"), "a");
}

#[test]
fn submission_marker() {
    assert_eq!(task_594::TRACK_PROOF, 0x52540000 | 594);
}
