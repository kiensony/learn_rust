use task_413::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        ":one:猫:ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        "bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            remainder_after_first_mark_colon(&mut data);
            data
        },
        "🐈"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_413::TRACK_PROOF, 0x52540000 | 413);
}
