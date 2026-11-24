use task_891::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_equals("hi=kit"), 5);
    assert_eq!(longest_field_with_last_tie_equals("=é=a"), 4);
    assert_eq!(longest_field_with_last_tie_equals(""), 2);
    assert_eq!(longest_field_with_last_tie_equals("="), 2);
    assert_eq!(longest_field_with_last_tie_equals("cat"), 5);
    assert_eq!(longest_field_with_last_tie_equals("==one=猫=ab"), 5);
    assert_eq!(longest_field_with_last_tie_equals("aa=bb"), 4);
    assert_eq!(longest_field_with_last_tie_equals("ok=🐈"), 6);
    assert_eq!(longest_field_with_last_tie_equals("aa=bb;meta=123"), 4);
    assert_eq!(longest_field_with_last_tie_equals(";ignore=all"), 2);
    assert_eq!(longest_field_with_last_tie_equals("a==b;"), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_891::TRACK_PROOF, 0x52540000 | 891);
}
