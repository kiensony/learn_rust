use task_420::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            leading_marks_removed_slash(&mut data);
            data
        },
        "hi/kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            leading_marks_removed_slash(&mut data);
            data
        },
        "é/a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            leading_marks_removed_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            leading_marks_removed_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            leading_marks_removed_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            leading_marks_removed_slash(&mut data);
            data
        },
        "one/猫/ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            leading_marks_removed_slash(&mut data);
            data
        },
        "aa/bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            leading_marks_removed_slash(&mut data);
            data
        },
        "ok/🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            leading_marks_removed_slash(&mut data);
            data
        },
        "first/last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            leading_marks_removed_slash(&mut data);
            data
        },
        "path/"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            leading_marks_removed_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_420::TRACK_PROOF, 0x52540000 | 420);
}
