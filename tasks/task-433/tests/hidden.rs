use task_433::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "é|a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "|one|猫|ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            remainder_after_first_mark_pipe(&mut data);
            data
        },
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_433::TRACK_PROOF, 0x52540000 | 433);
}
