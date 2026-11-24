use task_441::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            closing_marks_removed_equals(&mut data);
            data
        },
        "hi=kit"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            closing_marks_removed_equals(&mut data);
            data
        },
        "=é=a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_marks_removed_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            closing_marks_removed_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_marks_removed_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            closing_marks_removed_equals(&mut data);
            data
        },
        "==one=猫=ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            closing_marks_removed_equals(&mut data);
            data
        },
        "aa=bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            closing_marks_removed_equals(&mut data);
            data
        },
        "ok=🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            closing_marks_removed_equals(&mut data);
            data
        },
        "aa=bb"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            closing_marks_removed_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            closing_marks_removed_equals(&mut data);
            data
        },
        "a==b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_441::TRACK_PROOF, 0x52540000 | 441);
}
