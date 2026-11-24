use task_1009::*;

#[test]
fn edge_cases() {
    assert_eq!(first_nonempty_field_colon([""]), [0]);
    assert_eq!(
        first_nonempty_field_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 3, 3, 2, 2]
    );
    assert_eq!(
        first_nonempty_field_colon([
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
        [2, 2, 0, 0, 3, 3, 2, 2, 2]
    );
    assert_eq!(
        first_nonempty_field_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1009::TRACK_PROOF, 0x52540000 | 1009);
}
