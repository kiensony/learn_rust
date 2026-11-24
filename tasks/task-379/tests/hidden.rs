use task_379::*;

#[test]
fn edge_cases() {
    assert_eq!(closing_field_pipe(&String::from("header\n猫|123")), 3);
    assert_eq!(closing_field_pipe(&String::from("hi|kit")), 3);
    assert_eq!(closing_field_pipe(&String::from("")), 0);
    assert_eq!(closing_field_pipe(&String::from("|")), 0);
    assert_eq!(closing_field_pipe(&String::from("cat")), 3);
    assert_eq!(closing_field_pipe(&String::from("||one|猫|ab")), 2);
    assert_eq!(closing_field_pipe(&String::from("aa|bb")), 2);
    assert_eq!(closing_field_pipe(&String::from("ok|🐈")), 4);
    assert_eq!(
        closing_field_pipe(&String::from("header\nleft|right\r\n")),
        5
    );
    assert_eq!(closing_field_pipe(&String::from("header\n\n")), 0);
    assert_eq!(closing_field_pipe(&String::from("猫|x\nend|a")), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_379::TRACK_PROOF, 0x52540000 | 379);
}
