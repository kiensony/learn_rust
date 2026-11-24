use task_443::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "kit"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "é=a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "=one=猫=ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            remainder_after_first_mark_equals(&mut data);
            data
        },
        "=b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_443::TRACK_PROOF, 0x52540000 | 443);
}
