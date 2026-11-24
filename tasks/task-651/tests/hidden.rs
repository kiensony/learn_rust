use task_651::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("[猫###a]"),
            revision: 1
        }),
        4
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("[]"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("[42]"),
            revision: 4
        }),
        4
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("[321]"),
            revision: 5
        }),
        5
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("hi#kit"),
            revision: 6
        }),
        6
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("#é#a"),
            revision: 7
        }),
        7
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from(""),
            revision: 8
        }),
        8
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("#"),
            revision: 9
        }),
        9
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("cat"),
            revision: 10
        }),
        10
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("##one#猫#ab"),
            revision: 11
        }),
        11
    );
    assert_eq!(
        remainder_after_first_mark_hash(&Record {
            label: String::from("aa#bb"),
            revision: 12
        }),
        12
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_651::TRACK_PROOF, 0x52540000 | 651);
}
