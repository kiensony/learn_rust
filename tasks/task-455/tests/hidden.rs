use task_455::*;

#[test]
fn edge_cases() {
    assert_eq!(
        {
            let mut data = String::from("unclosed[cat");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[]");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("[42]");
            first_nonempty_field_hash(&mut data);
            data
        },
        "42"
    );
    assert_eq!(
        {
            let mut data = String::from("[321]");
            first_nonempty_field_hash(&mut data);
            data
        },
        "321"
    );
    assert_eq!(
        {
            let mut data = String::from("hi#kit");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#é#a");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("#");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("cat");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("##one#猫#ab");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
    assert_eq!(
        {
            let mut data = String::from("aa#bb");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_455::TRACK_PROOF, 0x52540000 | 455);
}
