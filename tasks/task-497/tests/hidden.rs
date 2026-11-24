use task_497::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_equals(("猫=12;note", "z=w")),
        2
    );
    assert_eq!(shortest_field_with_first_tie_equals(("hi=kit", "x")), 2);
    assert_eq!(shortest_field_with_first_tie_equals(("", "x")), 1);
    assert_eq!(shortest_field_with_first_tie_equals(("=", "z=w")), 1);
    assert_eq!(shortest_field_with_first_tie_equals(("cat", "x")), 3);
    assert_eq!(
        shortest_field_with_first_tie_equals(("==one=猫=ab", "z=w")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_equals(("aa=bb", "x")), 2);
    assert_eq!(shortest_field_with_first_tie_equals(("ok=🐈", "z=w")), 2);
    assert_eq!(
        shortest_field_with_first_tie_equals(("aa=bb;meta=123", "x")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_equals((";ignore=all", "z=w")),
        0
    );
    assert_eq!(shortest_field_with_first_tie_equals(("a==b;", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_497::TRACK_PROOF, 0x52540000 | 497);
}
