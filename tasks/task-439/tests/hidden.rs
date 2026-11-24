use task_439::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            closing_field_equals(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            closing_field_equals(&mut data);
            data
        },
        "a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            closing_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_field_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            closing_field_equals(&mut data);
            data
        },
        "ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            closing_field_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            closing_field_equals(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            closing_field_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            closing_field_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            closing_field_equals(&mut data);
            data
        },
        "b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_439::TRACK_PROOF, 0x52540000 | 439);
}
