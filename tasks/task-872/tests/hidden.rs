use task_872::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash("hi/kit"), 4);
    assert_eq!(shortest_field_with_first_tie_slash("/é/a"), 2);
    assert_eq!(shortest_field_with_first_tie_slash(""), 2);
    assert_eq!(shortest_field_with_first_tie_slash("/"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("cat"), 5);
    assert_eq!(shortest_field_with_first_tie_slash("//one/猫/ab"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("aa/bb"), 4);
    assert_eq!(shortest_field_with_first_tie_slash("ok/🐈"), 4);
    assert_eq!(shortest_field_with_first_tie_slash("path/first/last"), 6);
    assert_eq!(shortest_field_with_first_tie_slash("path/path/"), 2);
    assert_eq!(shortest_field_with_first_tie_slash("path/"), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_872::TRACK_PROOF, 0x52540000 | 872);
}
