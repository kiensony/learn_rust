use task_423::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "é/a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "/one/猫/ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        "last"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            remainder_after_first_mark_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_423::TRACK_PROOF, 0x52540000 | 423);
}
