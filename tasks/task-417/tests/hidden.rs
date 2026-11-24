use task_417::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            shortest_field_with_first_tie_colon(&mut data);
            data
        },
        "ok"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_417::TRACK_PROOF, 0x52540000 | 417);
}
