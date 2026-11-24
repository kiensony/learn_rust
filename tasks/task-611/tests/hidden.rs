use task_611::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from(""),
            revision: 2
        }),
        2
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from(":"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from("cat"),
            revision: 4
        }),
        7
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from("::one:猫:ab"),
            revision: 5
        }),
        15
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from("aa:bb"),
            revision: 6
        }),
        11
    );
    assert_eq!(
        marks_removed_at_both_ends_colon(&Record {
            label: String::from("ok:🐈"),
            revision: 7
        }),
        14
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_611::TRACK_PROOF, 0x52540000 | 611);
}
