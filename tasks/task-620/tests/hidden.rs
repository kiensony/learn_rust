use task_620::*;

#[test]
fn edge_cases() {
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("/é/a"),
            revision: 3
        }),
        7
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from(""),
            revision: 4
        }),
        4
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("/"),
            revision: 5
        }),
        5
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("cat"),
            revision: 6
        }),
        9
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("//one/猫/ab"),
            revision: 7
        }),
        17
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("aa/bb"),
            revision: 8
        }),
        13
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("ok/🐈"),
            revision: 9
        }),
        16
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("path/first/last"),
            revision: 10
        }),
        20
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("path/path/"),
            revision: 11
        }),
        15
    );
    assert_eq!(
        marks_removed_at_both_ends_slash(&Record {
            label: String::from("path/"),
            revision: 12
        }),
        12
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_620::TRACK_PROOF, 0x52540000 | 620);
}
