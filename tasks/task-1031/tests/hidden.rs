use task_1031::*;

#[test]
fn edge_cases() {
    assert_eq!(shortest_field_with_first_tie_pipe(["hi|kit"]), [2]);
    assert_eq!(
        shortest_field_with_first_tie_pipe([
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
        [0, 0, 0, 3, 0, 2, 2, 4, 0, 1]
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe([
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
        [3, 3, 2, 0, 0, 0, 3, 0, 2, 2, 4, 0, 1, 3]
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe([
            String::from(""),
            String::from("|"),
            String::from("|")
        ]),
        [0, 0, 0]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1031::TRACK_PROOF, 0x52540000 | 1031);
}
