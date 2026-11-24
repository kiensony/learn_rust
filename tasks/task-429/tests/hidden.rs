use task_429::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            closing_field_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            closing_field_pipe(&mut data);
            data
        },
        "a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            closing_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_field_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            closing_field_pipe(&mut data);
            data
        },
        "ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            closing_field_pipe(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            closing_field_pipe(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            closing_field_pipe(&mut data);
            data
        },
        "right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            closing_field_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            closing_field_pipe(&mut data);
            data
        },
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_429::TRACK_PROOF, 0x52540000 | 429);
}
