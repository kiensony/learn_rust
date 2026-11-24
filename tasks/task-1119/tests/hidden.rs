use task_1119::*;

#[test]
fn edge_cases() {
    assert_eq!(longest_field_with_last_tie_slash(&"hi/kit"), 3);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("/é/a")), 2);
    assert_eq!(longest_field_with_last_tie_slash(&""), 0);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("/")), 0);
    assert_eq!(longest_field_with_last_tie_slash(&"cat"), 3);
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("//one/猫/ab")),
        3
    );
    assert_eq!(longest_field_with_last_tie_slash(&"aa/bb"), 2);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("ok/🐈")), 4);
    assert_eq!(longest_field_with_last_tie_slash(&"path/first/last"), 5);
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/path/")),
        4
    );
    assert_eq!(longest_field_with_last_tie_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1119::TRACK_PROOF, 0x52540000 | 1119);
}
