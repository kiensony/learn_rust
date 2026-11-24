use task_547::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_equals("猫=12;note"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("hi=kit"), 2);
    assert_eq!(shortest_field_with_first_tie_equals(""), 0);
    assert_eq!(shortest_field_with_first_tie_equals("="), 0);
    assert_eq!(shortest_field_with_first_tie_equals("cat"), 3);
    assert_eq!(shortest_field_with_first_tie_equals("==one=猫=ab"), 0);
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("ok=🐈"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb;meta=123"), 2);
    assert_eq!(shortest_field_with_first_tie_equals(";ignore=all"), 0);
    assert_eq!(shortest_field_with_first_tie_equals("a==b;"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_547::TRACK_PROOF, 0x52540000 | 547);
}
