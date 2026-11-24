use task_437::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi|kit");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("|é|a");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("|");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("||one|猫|ab");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa|bb");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok|🐈");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("header\nleft|right\r\n");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "left"
    );
    assert_eq!(
        {
            let mut data = String::from("header\n\n");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("猫|x\nend|a");
            shortest_field_with_first_tie_pipe(&mut data);
            data
        },
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_437::TRACK_PROOF, 0x52540000 | 437);
}
