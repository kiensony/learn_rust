use task_576::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_slash("hi/kit"), "hi");
    assert_eq!(shortest_field_with_first_tie_slash("/é/a"), "");
    assert_eq!(shortest_field_with_first_tie_slash(""), "");
    assert_eq!(shortest_field_with_first_tie_slash("/"), "");
    assert_eq!(shortest_field_with_first_tie_slash("cat"), "cat");
    assert_eq!(shortest_field_with_first_tie_slash("//one/猫/ab"), "");
    assert_eq!(shortest_field_with_first_tie_slash("aa/bb"), "aa");
    assert_eq!(shortest_field_with_first_tie_slash("ok/🐈"), "ok");
    assert_eq!(
        shortest_field_with_first_tie_slash("path/first/last"),
        "last"
    );
    assert_eq!(shortest_field_with_first_tie_slash("path/path/"), "");
    assert_eq!(shortest_field_with_first_tie_slash("path/"), "");
}

#[test]
fn submission_marker() {
    assert_eq!(task_576::TRACK_PROOF, 0x52540000 | 576);
}
