use task_774::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_field_pipe(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        closing_field_pipe(&["first\n123", "first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([(String::from("123"), 3usize)])
    );
    assert_eq!(
        closing_field_pipe(&[
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
            (String::from("ab"), 1usize),
            (String::from("bb"), 1usize),
            (String::from("🐈"), 1usize),
            (String::from("right"), 1usize),
            (String::from("a"), 1usize)
        ])
    );
    assert_eq!(
        closing_field_pipe(&["", "", "|", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_774::TRACK_PROOF, 0x52540000 | 774);
}
