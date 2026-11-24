use task_409::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            closing_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            closing_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_field_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            closing_field_colon(&mut data);
            data
        },
        "ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            closing_field_colon(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            closing_field_colon(&mut data);
            data
        },
        "🐈"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_409::TRACK_PROOF, 0x52540000 | 409);
}
