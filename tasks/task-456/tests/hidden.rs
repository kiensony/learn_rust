use task_456::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("unclosed[cat");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[]");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[42]");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        "42"
    );
    assert_eq!(
        {
            let mut data = String::from("[321]");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        "321"
    );
    assert_eq!(
        {
            let mut data = String::from("hi#kit");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#é#a");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("##one#猫#ab");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa#bb");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_456::TRACK_PROOF, 0x52540000 | 456);
}
