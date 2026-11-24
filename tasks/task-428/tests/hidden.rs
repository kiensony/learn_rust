use task_428::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            leading_field_pipe(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            leading_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            leading_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            leading_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            leading_field_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            leading_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            leading_field_pipe(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            leading_field_pipe(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            leading_field_pipe(&mut data);
            data
        },
        "left"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            leading_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            leading_field_pipe(&mut data);
            data
        },
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_428::TRACK_PROOF, 0x52540000 | 428);
}
