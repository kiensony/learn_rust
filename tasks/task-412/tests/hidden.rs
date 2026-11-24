use task_412::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "one:猫:ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "aa:bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            marks_removed_at_both_ends_colon(&mut data);
            data
        },
        "ok:🐈"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_412::TRACK_PROOF, 0x52540000 | 412);
}
