use task_892::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_equals("猫=12;note"), 4);
    assert_eq!(shortest_field_with_first_tie_equals("hi=kit"), 4);
    assert_eq!(shortest_field_with_first_tie_equals(""), 2);
    assert_eq!(shortest_field_with_first_tie_equals("="), 2);
    assert_eq!(shortest_field_with_first_tie_equals("cat"), 5);
    assert_eq!(shortest_field_with_first_tie_equals("==one=猫=ab"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb"), 4);
    assert_eq!(shortest_field_with_first_tie_equals("ok=🐈"), 4);
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb;meta=123"), 4);
    assert_eq!(shortest_field_with_first_tie_equals(";ignore=all"), 2);
    assert_eq!(shortest_field_with_first_tie_equals("a==b;"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_892::TRACK_PROOF, 0x52540000 | 892);
}
