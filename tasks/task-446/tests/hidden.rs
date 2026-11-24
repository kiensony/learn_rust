use task_446::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "é"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "猫"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            longest_field_with_last_tie_equals(&mut data);
            data
        },
        "b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_446::TRACK_PROOF, 0x52540000 | 446);
}
