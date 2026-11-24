use task_1007::*;

#[test]
fn edge_cases() {
    assert_eq!(remainder_after_first_mark_colon([""]), [0]);
    assert_eq!(
        remainder_after_first_mark_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 0, 11, 2, 4]
    );
    assert_eq!(
        remainder_after_first_mark_colon([
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
        [3, 4, 0, 0, 0, 11, 2, 4, 3]
    );
    assert_eq!(
        remainder_after_first_mark_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1007::TRACK_PROOF, 0x52540000 | 1007);
}
