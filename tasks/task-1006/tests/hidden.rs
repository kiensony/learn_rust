use task_1006::*;

#[test]
fn edge_cases() {
    assert_eq!(marks_removed_at_both_ends_colon([""]), [0]);
    assert_eq!(
        marks_removed_at_both_ends_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 3, 10, 5, 7]
    );
    assert_eq!(
        marks_removed_at_both_ends_colon([
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
        [6, 4, 0, 0, 3, 10, 5, 7, 6]
    );
    assert_eq!(
        marks_removed_at_both_ends_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1006::TRACK_PROOF, 0x52540000 | 1006);
}
