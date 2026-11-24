use task_442::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("hi=kit");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "hi=kit"
    );
    assert_eq!(
        {
            let mut data = String::from("=é=a");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "é=a"
    );
    assert_eq!(
        {
            let mut data = String::from("");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("=");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("==one=猫=ab");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "one=猫=ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "aa=bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok=🐈");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "ok=🐈"
    );
    assert_eq!(
        {
            let mut data = String::from("aa=bb;meta=123");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "aa=bb"
    );
    assert_eq!(
        {
            let mut data = String::from(";ignore=all");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("a==b;");
            marks_removed_at_both_ends_equals(&mut data);
            data
        },
        "a==b"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_442::TRACK_PROOF, 0x52540000 | 442);
}
