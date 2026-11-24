use task_1139::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_equals(&"hi=kit"), 3);
    assert_eq!(longest_field_with_last_tie_equals(&String::from("=é=a")), 2);
    assert_eq!(longest_field_with_last_tie_equals(&""), 0);
    assert_eq!(longest_field_with_last_tie_equals(&String::from("=")), 0);
    assert_eq!(longest_field_with_last_tie_equals(&"cat"), 3);
    assert_eq!(
        longest_field_with_last_tie_equals(&String::from("==one=猫=ab")),
        3
    );
    assert_eq!(longest_field_with_last_tie_equals(&"aa=bb"), 2);
    assert_eq!(
        longest_field_with_last_tie_equals(&String::from("ok=🐈")),
        4
    );
    assert_eq!(longest_field_with_last_tie_equals(&"aa=bb;meta=123"), 2);
    assert_eq!(
        longest_field_with_last_tie_equals(&String::from(";ignore=all")),
        0
    );
    assert_eq!(longest_field_with_last_tie_equals(&"a==b;"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1139::TRACK_PROOF, 0x52540000 | 1139);
}
