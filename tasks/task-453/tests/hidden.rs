use task_453::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("unclosed[cat");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[]");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[42]");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[321]");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("hi#kit");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#é#a");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("##one#猫#ab");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa#bb");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_453::TRACK_PROOF, 0x52540000 | 453);
}
