use task_327::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("hi/kit")),
        "hi"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("/é/a")),
        ""
    );
    assert_eq!(shortest_field_with_first_tie_slash(String::from("")), "");
    assert_eq!(shortest_field_with_first_tie_slash(String::from("/")), "");
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("cat")),
        "cat"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("//one/猫/ab")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("aa/bb")),
        "aa"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("ok/🐈")),
        "ok"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("path/first/last")),
        "last"
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("path/path/")),
        ""
    );
    assert_eq!(
        shortest_field_with_first_tie_slash(String::from("path/")),
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_327::TRACK_PROOF, 0x52540000 | 327);
}
