use task_431::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "hi|kit"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "|é|a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            closing_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "||one|猫|ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "aa|bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "ok|🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "left|right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            closing_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            closing_marks_removed_pipe(&mut data);
            data
        },
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_431::TRACK_PROOF, 0x52540000 | 431);
}
