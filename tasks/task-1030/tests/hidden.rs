use task_1030::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe([String::from("header\n猫|123"), String::from("hi|kit")]),
        [3, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe([
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
        [2, 0, 0, 3, 3, 2, 4, 5, 0, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe([
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
        [3, 3, 3, 2, 0, 0, 3, 3, 2, 4, 5, 0, 3, 3]
    );
    assert_eq!(
        longest_field_with_last_tie_pipe([String::from(""), String::from("|"), String::from("|")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1030::TRACK_PROOF, 0x52540000 | 1030);
}
