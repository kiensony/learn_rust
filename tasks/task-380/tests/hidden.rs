use task_380::*;

#[test]
fn edge_cases() {
    assert_eq!(leading_marks_removed_pipe(&String::from("hi|kit")), 6);
    assert_eq!(leading_marks_removed_pipe(&String::from("|é|a")), 4);
    assert_eq!(leading_marks_removed_pipe(&String::from("")), 0);
    assert_eq!(leading_marks_removed_pipe(&String::from("|")), 0);
    assert_eq!(leading_marks_removed_pipe(&String::from("cat")), 3);
    assert_eq!(leading_marks_removed_pipe(&String::from("||one|猫|ab")), 10);
    assert_eq!(leading_marks_removed_pipe(&String::from("aa|bb")), 5);
    assert_eq!(leading_marks_removed_pipe(&String::from("ok|🐈")), 7);
    assert_eq!(
        leading_marks_removed_pipe(&String::from("header\nleft|right\r\n")),
        10
    );
    assert_eq!(leading_marks_removed_pipe(&String::from("header\n\n")), 0);
    assert_eq!(leading_marks_removed_pipe(&String::from("猫|x\nend|a")), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_380::TRACK_PROOF, 0x52540000 | 380);
}
