use task_1023::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_field_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [3, 3]
    );
    assert_eq!(
        closing_field_pipe([
            String::from("|é|a"),
            String::from(""),
            String::from("|"),
            String::from("cat"),
            String::from("||one|猫|ab"),
            String::from("aa|bb"),
            String::from("ok|🐈"),
            String::from("header\nleft|right\r\n"),
            String::from("header\n\n"),
            String::from("猫|x\nend|a")
        ]),
        [1, 0, 0, 3, 2, 2, 4, 5, 0, 1]
    );
    assert_eq!(
        closing_field_pipe([
            "first\n123",
            "header\n猫|123",
            "hi|kit",
            "|é|a",
            "",
            "|",
            "cat",
            "||one|猫|ab",
            "aa|bb",
            "ok|🐈",
            "header\nleft|right\r\n",
            "header\n\n",
            "猫|x\nend|a",
            "first\n123"
        ]),
        [3, 3, 3, 1, 0, 0, 3, 2, 2, 4, 5, 0, 1, 3]
    );
    assert_eq!(
        closing_field_pipe([String::from(""), String::from("|"), String::from("|")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1023::TRACK_PROOF, 0x52540000 | 1023);
}
