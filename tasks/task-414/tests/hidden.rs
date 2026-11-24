use task_414::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        "::one:猫"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        "aa"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            prefix_before_final_mark_colon(&mut data);
            data
        },
        "ok"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_414::TRACK_PROOF, 0x52540000 | 414);
}
