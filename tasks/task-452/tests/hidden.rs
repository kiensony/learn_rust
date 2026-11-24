use task_452::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("unclosed[cat");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[]");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[42]");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        "42"
    );
    assert_eq!(
        {
            let mut data = String::from("[321]");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        "321"
    );
    assert_eq!(
        {
            let mut data = String::from("hi#kit");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#é#a");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("##one#猫#ab");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa#bb");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            marks_removed_at_both_ends_hash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_452::TRACK_PROOF, 0x52540000 | 452);
}
