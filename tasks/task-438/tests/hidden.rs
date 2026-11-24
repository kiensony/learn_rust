use task_438::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            leading_field_equals(&mut data);
            data
        },
        "hi"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            leading_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            leading_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            leading_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            leading_field_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            leading_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            leading_field_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            leading_field_equals(&mut data);
            data
        },
        "ok"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            leading_field_equals(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            leading_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            leading_field_equals(&mut data);
            data
        },
        "a"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_438::TRACK_PROOF, 0x52540000 | 438);
}
