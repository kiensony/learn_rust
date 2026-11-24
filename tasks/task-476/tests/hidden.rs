use task_476::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash(("hi/kit", "x")), 3);
    assert_eq!(longest_field_with_last_tie_slash(("/é/a", "z/w")), 2);
    assert_eq!(longest_field_with_last_tie_slash(("", "x")), 1);
    assert_eq!(longest_field_with_last_tie_slash(("/", "z/w")), 1);
    assert_eq!(longest_field_with_last_tie_slash(("cat", "x")), 3);
    assert_eq!(longest_field_with_last_tie_slash(("//one/猫/ab", "z/w")), 3);
    assert_eq!(longest_field_with_last_tie_slash(("aa/bb", "x")), 2);
    assert_eq!(longest_field_with_last_tie_slash(("ok/🐈", "z/w")), 4);
    assert_eq!(
        longest_field_with_last_tie_slash(("path/first/last", "x")),
        5
    );
    assert_eq!(longest_field_with_last_tie_slash(("path/path/", "z/w")), 4);
    assert_eq!(longest_field_with_last_tie_slash(("path/", "x")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_476::TRACK_PROOF, 0x52540000 | 476);
}
