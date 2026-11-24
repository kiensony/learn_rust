use task_419::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            closing_field_slash(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            closing_field_slash(&mut data);
            data
        },
        "a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_field_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            closing_field_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_field_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            closing_field_slash(&mut data);
            data
        },
        "ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            closing_field_slash(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            closing_field_slash(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            closing_field_slash(&mut data);
            data
        },
        "last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            closing_field_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            closing_field_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_419::TRACK_PROOF, 0x52540000 | 419);
}
