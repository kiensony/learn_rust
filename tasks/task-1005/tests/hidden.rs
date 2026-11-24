use task_1005::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_marks_removed_colon([""]), [0]);
    assert_eq!(
        closing_marks_removed_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 3, 12, 5, 7]
    );
    assert_eq!(
        closing_marks_removed_colon([
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
        [6, 5, 0, 0, 3, 12, 5, 7, 6]
    );
    assert_eq!(
        closing_marks_removed_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1005::TRACK_PROOF, 0x52540000 | 1005);
}
