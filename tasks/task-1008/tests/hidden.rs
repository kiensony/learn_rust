use task_1008::*;

#[test]
fn edge_cases() {
    assert_eq!(prefix_before_final_mark_colon([""]), [0]);
    assert_eq!(
        prefix_before_final_mark_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 0, 9, 2, 2]
    );
    assert_eq!(
        prefix_before_final_mark_colon([
            "hi:kit",
            ":é:a",
            "",
            ":",
            "cat",
            "::one:猫:ab",
            "aa:bb",
            "ok:🐈",
            "hi:kit"
        ]),
        [2, 3, 0, 0, 0, 9, 2, 2, 2]
    );
    assert_eq!(
        prefix_before_final_mark_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1008::TRACK_PROOF, 0x52540000 | 1008);
}
