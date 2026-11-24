use task_421::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            closing_marks_removed_slash(&mut data);
            data
        },
        "hi/kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            closing_marks_removed_slash(&mut data);
            data
        },
        "/é/a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_marks_removed_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            closing_marks_removed_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_marks_removed_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            closing_marks_removed_slash(&mut data);
            data
        },
        "//one/猫/ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            closing_marks_removed_slash(&mut data);
            data
        },
        "aa/bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            closing_marks_removed_slash(&mut data);
            data
        },
        "ok/🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            closing_marks_removed_slash(&mut data);
            data
        },
        "first/last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            closing_marks_removed_slash(&mut data);
            data
        },
        "path"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            closing_marks_removed_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_421::TRACK_PROOF, 0x52540000 | 421);
}
