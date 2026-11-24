use task_449::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("unclosed[cat");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[]");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[42]");
            closing_field_hash(&mut data);
            data
        },
        "42"
    );
    assert_eq!(
        {
            let mut data = String::from("[321]");
            closing_field_hash(&mut data);
            data
        },
        "321"
    );
    assert_eq!(
        {
            let mut data = String::from("hi#kit");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#é#a");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("##one#猫#ab");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa#bb");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            closing_field_hash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_449::TRACK_PROOF, 0x52540000 | 449);
}
