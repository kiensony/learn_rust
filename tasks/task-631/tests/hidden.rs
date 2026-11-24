use task_631::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("hi|kit"),
            revision: 2
        }),
        5
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("|é|a"),
            revision: 3
        }),
        7
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from(""),
            revision: 4
        }),
        4
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("|"),
            revision: 5
        }),
        5
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("cat"),
            revision: 6
        }),
        6
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("||one|猫|ab"),
            revision: 7
        }),
        18
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("aa|bb"),
            revision: 8
        }),
        10
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("ok|🐈"),
            revision: 9
        }),
        13
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("header\nleft|right\r\n"),
            revision: 10
        }),
        15
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("header\n\n"),
            revision: 11
        }),
        11
    );
    assert_eq!(
        remainder_after_first_mark_pipe(&Record {
            label: String::from("猫|x\nend|a"),
            revision: 12
        }),
        13
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_631::TRACK_PROOF, 0x52540000 | 631);
}
