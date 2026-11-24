use task_422::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "hi/kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "é/a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "one/猫/ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "aa/bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "ok/🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "first/last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        "path"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            marks_removed_at_both_ends_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_422::TRACK_PROOF, 0x52540000 | 422);
}
