use task_415::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            first_nonempty_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            first_nonempty_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            first_nonempty_field_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            first_nonempty_field_colon(&mut data);
            data
        },
        "one"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            first_nonempty_field_colon(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            first_nonempty_field_colon(&mut data);
            data
        },
        "ok"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_415::TRACK_PROOF, 0x52540000 | 415);
}
