use task_780::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_nonempty_field_pipe(&[]),
        std::collections::BTreeMap::from([])
    );
    assert_eq!(
        first_nonempty_field_pipe(&["first\n123", "first\n123", "header\n猫|123"]),
        std::collections::BTreeMap::from([
            (String::from("123"), 2usize),
            (String::from("猫"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_pipe(&[
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
            (String::from("one"), 1usize),
            (String::from("aa"), 1usize),
            (String::from("ok"), 1usize),
            (String::from("left"), 1usize),
            (String::from("end"), 1usize)
        ])
    );
    assert_eq!(
        first_nonempty_field_pipe(&["", "", "|", ""]),
        std::collections::BTreeMap::from([(String::from(""), 4usize)])
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_780::TRACK_PROOF, 0x52540000 | 780);
}
