use task_610::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from(":é:a"),
            revision: 1
        }),
        6
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from(":"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from("cat"),
            revision: 4
        }),
        7
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from("::one:猫:ab"),
            revision: 5
        }),
        17
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from("aa:bb"),
            revision: 6
        }),
        11
    );
    assert_eq!(
        closing_marks_removed_colon(&Record {
            label: String::from("ok:🐈"),
            revision: 7
        }),
        14
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_610::TRACK_PROOF, 0x52540000 | 610);
}
