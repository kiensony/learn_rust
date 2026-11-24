use task_408::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            leading_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            leading_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            leading_field_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            leading_field_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            leading_field_colon(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            leading_field_colon(&mut data);
            data
        },
        "ok"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_408::TRACK_PROOF, 0x52540000 | 408);
}
