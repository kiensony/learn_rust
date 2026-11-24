use task_1024::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe(["hi|kit"]), [6]);
    assert_eq!(
        leading_marks_removed_pipe([
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
        [4, 0, 0, 3, 10, 5, 7, 10, 0, 5]
    );
    assert_eq!(
        leading_marks_removed_pipe([
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
        [3, 7, 6, 4, 0, 0, 3, 10, 5, 7, 10, 0, 5, 3]
    );
    assert_eq!(
        leading_marks_removed_pipe([String::from(""), String::from("|"), String::from("|")]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1024::TRACK_PROOF, 0x52540000 | 1024);
}
