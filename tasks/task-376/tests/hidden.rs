use task_376::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("hi/kit")),
        3
    );
    assert_eq!(longest_field_with_last_tie_slash(&String::from("/é/a")), 2);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("")), 0);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("/")), 0);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("cat")), 3);
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("//one/猫/ab")),
        3
    );
    assert_eq!(longest_field_with_last_tie_slash(&String::from("aa/bb")), 2);
    assert_eq!(longest_field_with_last_tie_slash(&String::from("ok/🐈")), 4);
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/first/last")),
        5
    );
    assert_eq!(
        longest_field_with_last_tie_slash(&String::from("path/path/")),
        4
    );
    assert_eq!(longest_field_with_last_tie_slash(&String::from("path/")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_376::TRACK_PROOF, 0x52540000 | 376);
}
