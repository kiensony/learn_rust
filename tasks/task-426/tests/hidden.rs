use task_426::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "猫"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "first"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        "path"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            longest_field_with_last_tie_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_426::TRACK_PROOF, 0x52540000 | 426);
}
