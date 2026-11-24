use task_424::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi/kit");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("/é/a");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "/é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("/");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("//one/猫/ab");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "//one/猫"
    );
    assert_eq!(
        {
            let mut data = String::from("aa/bb");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok/🐈");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("path/first/last");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "first"
    );
    assert_eq!(
        {
            let mut data = String::from("path/path/");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        "path"
    );
    assert_eq!(
        {
            let mut data = String::from("path/");
            prefix_before_final_mark_slash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_424::TRACK_PROOF, 0x52540000 | 424);
}
