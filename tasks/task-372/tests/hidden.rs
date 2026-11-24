use task_372::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("hi/kit")), 6);
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("/é/a")), 4);
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("")), 0);
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("/")), 0);
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("cat")), 3);
    assert_eq!(
        marks_removed_at_both_ends_slash(&String::from("//one/猫/ab")),
        10
    );
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("aa/bb")), 5);
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("ok/🐈")), 7);
    assert_eq!(
        marks_removed_at_both_ends_slash(&String::from("path/first/last")),
        10
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&String::from("path/path/")),
        4
    );
    assert_eq!(marks_removed_at_both_ends_slash(&String::from("path/")), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_372::TRACK_PROOF, 0x52540000 | 372);
}
