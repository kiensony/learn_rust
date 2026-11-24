use task_781::*;

#[test]
fn edge_cases() {
    assert_eq!(
        longest_field_with_last_tie_pipe(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(&["first\n123", "first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([(String::from("123"), 3usize)])
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(&[
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
            (String::from("猫"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize),
            (String::from("right"), 1usize),
            (String::from("end"), 1usize)
        ])
    );
    assert_eq!(
        longest_field_with_last_tie_pipe(&["", "", "|", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_781::TRACK_PROOF, 0x52540000 | 781);
}
