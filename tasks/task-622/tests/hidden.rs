use task_622::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("path/123"),
            revision: 1
        }),
        1
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("/é/a"),
            revision: 3
        }),
        6
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from(""),
            revision: 4
        }),
        4
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("/"),
            revision: 5
        }),
        5
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("cat"),
            revision: 6
        }),
        6
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("//one/猫/ab"),
            revision: 7
        }),
        16
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("aa/bb"),
            revision: 8
        }),
        10
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("ok/🐈"),
            revision: 9
        }),
        11
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("path/first/last"),
            revision: 10
        }),
        15
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("path/path/"),
            revision: 11
        }),
        15
    );
    assert_eq!(
        prefix_before_final_mark_slash(&Record {
            label: String::from("path/"),
            revision: 12
        }),
        12
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_622::TRACK_PROOF, 0x52540000 | 622);
}
