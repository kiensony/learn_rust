use task_595::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_equals("hi=kit"), "kit");
    assert_eq!(longest_field_with_last_tie_equals("=é=a"), "é");
    assert_eq!(longest_field_with_last_tie_equals(""), "");
    assert_eq!(longest_field_with_last_tie_equals("="), "");
    assert_eq!(longest_field_with_last_tie_equals("cat"), "cat");
    assert_eq!(longest_field_with_last_tie_equals("==one=猫=ab"), "猫");
    assert_eq!(longest_field_with_last_tie_equals("aa=bb"), "bb");
    assert_eq!(longest_field_with_last_tie_equals("ok=🐈"), "🐈");
    assert_eq!(longest_field_with_last_tie_equals("aa=bb;meta=123"), "bb");
    assert_eq!(longest_field_with_last_tie_equals(";ignore=all"), "");
    assert_eq!(longest_field_with_last_tie_equals("a==b;"), "b");
}

#[test]
fn submission_marker() {
    assert_eq!(task_595::TRACK_PROOF, 0x52540000 | 595);
}
