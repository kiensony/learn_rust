use task_445::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            first_nonempty_field_equals(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            first_nonempty_field_equals(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            first_nonempty_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            first_nonempty_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            first_nonempty_field_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            first_nonempty_field_equals(&mut data);
            data
        },
        "one"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            first_nonempty_field_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            first_nonempty_field_equals(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            first_nonempty_field_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            first_nonempty_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            first_nonempty_field_equals(&mut data);
            data
        },
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_445::TRACK_PROOF, 0x52540000 | 445);
}
