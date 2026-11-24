use task_427::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_427::TRACK_PROOF, 0x52540000 | 427);
}
