use task_596::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_equals("hi=kit"), "hi");
    assert_eq!(shortest_field_with_first_tie_equals("=é=a"), "");
    assert_eq!(shortest_field_with_first_tie_equals(""), "");
    assert_eq!(shortest_field_with_first_tie_equals("="), "");
    assert_eq!(shortest_field_with_first_tie_equals("cat"), "cat");
    assert_eq!(shortest_field_with_first_tie_equals("==one=猫=ab"), "");
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb"), "aa");
    assert_eq!(shortest_field_with_first_tie_equals("ok=🐈"), "ok");
    assert_eq!(shortest_field_with_first_tie_equals("aa=bb;meta=123"), "aa");
    assert_eq!(shortest_field_with_first_tie_equals(";ignore=all"), "");
    assert_eq!(shortest_field_with_first_tie_equals("a==b;"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_596::TRACK_PROOF, 0x52540000 | 596);
}
