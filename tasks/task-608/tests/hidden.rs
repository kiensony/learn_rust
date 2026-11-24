use task_608::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from(""),
            revision: 2
        }),
        2
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from(":"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from("cat"),
            revision: 4
        }),
        7
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from("::one:猫:ab"),
            revision: 5
        }),
        7
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from("aa:bb"),
            revision: 6
        }),
        8
    );
    assert_eq!(
        closing_field_colon(&Record {
            label: String::from("ok:🐈"),
            revision: 7
        }),
        11
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_608::TRACK_PROOF, 0x52540000 | 608);
}
