use task_1003::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_colon([""]), [0]);
    assert_eq!(
        closing_field_colon([
            String::from(":"),
            String::from("cat"),
            String::from("::one:猫:ab"),
            String::from("aa:bb"),
            String::from("ok:🐈")
        ]),
        [0, 3, 2, 2, 4]
    );
    assert_eq!(
        closing_field_colon([
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
        [3, 1, 0, 0, 3, 2, 2, 4, 3]
    );
    assert_eq!(
        closing_field_colon([String::from(""), String::from(":"), String::from(":")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1003::TRACK_PROOF, 0x52540000 | 1003);
}
