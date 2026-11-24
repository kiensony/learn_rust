use task_775::*;

#[test]
fn edge_cases() {
    assert_eq!(
        leading_marks_removed_pipe(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        leading_marks_removed_pipe(&["first\n123", "first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 2usize),
            (String::from("猫|123"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_pipe(&[
            "",
            "|",
            "cat",
            "||one|猫|ab",
            "aa|bb",
            "ok|🐈",
            "header\nleft|right\r\n",
            "header\n\n",
            "猫|x\nend|a"
        ]),
        std::collections::BTreeMap::from([
            (String::from(""), 3usize),
            (String::from("cat"), 1usize),
            (String::from("one|猫|ab"), 1usize),
            (String::from("aa|bb"), 1usize),
            (String::from("ok|🐈"), 1usize),
            (String::from("left|right"), 1usize),
            (String::from("end|a"), 1usize)
        ])
    );
    assert_eq!(
        leading_marks_removed_pipe(&["", "", "|", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_775::TRACK_PROOF, 0x52540000 | 775);
}
