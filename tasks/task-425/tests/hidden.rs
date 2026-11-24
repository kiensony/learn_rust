use task_425::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            first_nonempty_field_slash(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            first_nonempty_field_slash(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            first_nonempty_field_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            first_nonempty_field_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            first_nonempty_field_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            first_nonempty_field_slash(&mut data);
            data
        },
        "one"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            first_nonempty_field_slash(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            first_nonempty_field_slash(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            first_nonempty_field_slash(&mut data);
            data
        },
        "first"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            first_nonempty_field_slash(&mut data);
            data
        },
        "path"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            first_nonempty_field_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_425::TRACK_PROOF, 0x52540000 | 425);
}
