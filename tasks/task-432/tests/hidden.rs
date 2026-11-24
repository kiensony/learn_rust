use task_432::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "hi|kit"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "é|a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "one|猫|ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "aa|bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "ok|🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "left|right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            marks_removed_at_both_ends_pipe(&mut data);
            data
        },
        "end|a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_432::TRACK_PROOF, 0x52540000 | 432);
}
