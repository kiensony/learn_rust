use task_527::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash("hi/kit"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("/é/a"), 0);
    assert_eq!(shortest_field_with_first_tie_slash(""), 0);
    assert_eq!(shortest_field_with_first_tie_slash("/"), 0);
    assert_eq!(shortest_field_with_first_tie_slash("cat"), 3);
    assert_eq!(shortest_field_with_first_tie_slash("//one/猫/ab"), 0);
    assert_eq!(shortest_field_with_first_tie_slash("aa/bb"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("ok/🐈"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("path/first/last"), 4);
    assert_eq!(shortest_field_with_first_tie_slash("path/path/"), 0);
    assert_eq!(shortest_field_with_first_tie_slash("path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_527::TRACK_PROOF, 0x52540000 | 527);
}
