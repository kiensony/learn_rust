use task_347::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("hi=kit")),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("=é=a")),
        ""
    );
    assert_eq!(shortest_field_with_first_tie_equals(String::from("")), "");
    assert_eq!(shortest_field_with_first_tie_equals(String::from("=")), "");
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("cat")),
        "cat"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("==one=猫=ab")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("aa=bb")),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("ok=🐈")),
        "ok"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("aa=bb;meta=123")),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from(";ignore=all")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_equals(String::from("a==b;")),
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_347::TRACK_PROOF, 0x52540000 | 347);
}
