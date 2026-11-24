use task_435::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            first_nonempty_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            first_nonempty_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "one"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "left"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            first_nonempty_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            first_nonempty_field_pipe(&mut data);
            data
        },
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_435::TRACK_PROOF, 0x52540000 | 435);
}
