use task_1067::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_slash(&"hi/kit"), 2);
    assert_eq!(prefix_before_final_mark_slash(&String::from("/é/a")), 3);
    assert_eq!(prefix_before_final_mark_slash(&""), 0);
    assert_eq!(prefix_before_final_mark_slash(&String::from("/")), 0);
    assert_eq!(prefix_before_final_mark_slash(&"cat"), 0);
    assert_eq!(
        prefix_before_final_mark_slash(&String::from("//one/猫/ab")),
        9
    );
    assert_eq!(prefix_before_final_mark_slash(&"aa/bb"), 2);
    assert_eq!(prefix_before_final_mark_slash(&String::from("ok/🐈")), 2);
    assert_eq!(prefix_before_final_mark_slash(&"path/first/last"), 5);
    assert_eq!(
        prefix_before_final_mark_slash(&String::from("path/path/")),
        4
    );
    assert_eq!(prefix_before_final_mark_slash(&"path/"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1067::TRACK_PROOF, 0x52540000 | 1067);
}
