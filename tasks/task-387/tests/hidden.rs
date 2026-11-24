use task_387::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("header\n猫|123")),
        3
    );
    assert_eq!(shortest_field_with_first_tie_pipe(&String::from("|é|a")), 0);
    assert_eq!(shortest_field_with_first_tie_pipe(&String::from("")), 0);
    assert_eq!(shortest_field_with_first_tie_pipe(&String::from("|")), 0);
    assert_eq!(shortest_field_with_first_tie_pipe(&String::from("cat")), 3);
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("||one|猫|ab")),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("aa|bb")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("ok|🐈")),
        2
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("header\nleft|right\r\n")),
        4
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("header\n\n")),
        0
    );
    assert_eq!(
        shortest_field_with_first_tie_pipe(&String::from("猫|x\nend|a")),
        1
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_387::TRACK_PROOF, 0x52540000 | 387);
}
