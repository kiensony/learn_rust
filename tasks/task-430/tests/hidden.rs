use task_430::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "hi|kit"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "é|a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            leading_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            leading_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "one|猫|ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "aa|bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "ok|🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "left|right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            leading_marks_removed_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            leading_marks_removed_pipe(&mut data);
            data
        },
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_430::TRACK_PROOF, 0x52540000 | 430);
}
