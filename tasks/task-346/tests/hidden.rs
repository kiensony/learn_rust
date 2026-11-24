use task_346::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("hi=kit")),
        "kit"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("=é=a")),
        "é"
    );
    assert_eq!(longest_field_with_last_tie_equals(String::from("")), "");
    assert_eq!(longest_field_with_last_tie_equals(String::from("=")), "");
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("cat")),
        "cat"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("==one=猫=ab")),
        "猫"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("aa=bb")),
        "bb"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("ok=🐈")),
        "🐈"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("aa=bb;meta=123")),
        "bb"
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from(";ignore=all")),
        ""
    );
    assert_eq!(
        longest_field_with_last_tie_equals(String::from("a==b;")),
        "b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_346::TRACK_PROOF, 0x52540000 | 346);
}
