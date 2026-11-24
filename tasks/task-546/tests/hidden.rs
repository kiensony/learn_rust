use task_546::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_equals("hi=kit"), 3);
    assert_eq!(longest_field_with_last_tie_equals("=é=a"), 1);
    assert_eq!(longest_field_with_last_tie_equals(""), 0);
    assert_eq!(longest_field_with_last_tie_equals("="), 0);
    assert_eq!(longest_field_with_last_tie_equals("cat"), 3);
    assert_eq!(longest_field_with_last_tie_equals("==one=猫=ab"), 1);
    assert_eq!(longest_field_with_last_tie_equals("aa=bb"), 2);
    assert_eq!(longest_field_with_last_tie_equals("ok=🐈"), 1);
    assert_eq!(longest_field_with_last_tie_equals("aa=bb;meta=123"), 2);
    assert_eq!(longest_field_with_last_tie_equals(";ignore=all"), 0);
    assert_eq!(longest_field_with_last_tie_equals("a==b;"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_546::TRACK_PROOF, 0x52540000 | 546);
}
