use task_1116::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_slash(&"hi/kit"), 3);
    assert_eq!(remainder_after_first_mark_slash(&String::from("/é/a")), 4);
    assert_eq!(remainder_after_first_mark_slash(&""), 0);
    assert_eq!(remainder_after_first_mark_slash(&String::from("/")), 0);
    assert_eq!(remainder_after_first_mark_slash(&"cat"), 0);
    assert_eq!(
        remainder_after_first_mark_slash(&String::from("//one/猫/ab")),
        11
    );
    assert_eq!(remainder_after_first_mark_slash(&"aa/bb"), 2);
    assert_eq!(remainder_after_first_mark_slash(&String::from("ok/🐈")), 4);
    assert_eq!(remainder_after_first_mark_slash(&"path/first/last"), 4);
    assert_eq!(
        remainder_after_first_mark_slash(&String::from("path/path/")),
        0
    );
    assert_eq!(remainder_after_first_mark_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1116::TRACK_PROOF, 0x52540000 | 1116);
}
