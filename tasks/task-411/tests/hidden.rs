use task_411::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("");
            closing_marks_removed_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from(":");
            closing_marks_removed_colon(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_marks_removed_colon(&mut data);
            data
        },
        "cat"
    );
    assert_eq!(
        {
            let mut data = String::from("::one:猫:ab");
            closing_marks_removed_colon(&mut data);
            data
        },
        "::one:猫:ab"
    );
    assert_eq!(
        {
            let mut data = String::from("aa:bb");
            closing_marks_removed_colon(&mut data);
            data
        },
        "aa:bb"
    );
    assert_eq!(
        {
            let mut data = String::from("ok:🐈");
            closing_marks_removed_colon(&mut data);
            data
        },
        "ok:🐈"
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_411::TRACK_PROOF, 0x52540000 | 411);
}
