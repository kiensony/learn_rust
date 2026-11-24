use task_612::*;

#[test]
fn edge_cases() {
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from(""),
            revision: 2
        }),
        2
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from(":"),
            revision: 3
        }),
        3
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from("cat"),
            revision: 4
        }),
        4
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from("::one:猫:ab"),
            revision: 5
        }),
        14
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from("aa:bb"),
            revision: 6
        }),
        8
    );
    assert_eq!(
        prefix_before_final_mark_colon(&Record {
            label: String::from("ok:🐈"),
            revision: 7
        }),
        9
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_612::TRACK_PROOF, 0x52540000 | 612);
}
