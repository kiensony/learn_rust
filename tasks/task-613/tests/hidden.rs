use task_613::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from(""),
            revision: 2
        }),
        2
    );
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from(":"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from("cat"),
            revision: 4
        }),
        7
    );
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from("::one:猫:ab"),
            revision: 5
        }),
        8
    );
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from("aa:bb"),
            revision: 6
        }),
        8
    );
    assert_eq!(
        first_nonempty_field_colon(&Record {
            label: String::from("ok:🐈"),
            revision: 7
        }),
        9
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_613::TRACK_PROOF, 0x52540000 | 613);
}
