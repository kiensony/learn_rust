use task_436::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("header\n猫|123");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "123"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "猫"
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "right"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            longest_field_with_last_tie_pipe(&mut data);
            data
        },
        "end"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_436::TRACK_PROOF, 0x52540000 | 436);
}
