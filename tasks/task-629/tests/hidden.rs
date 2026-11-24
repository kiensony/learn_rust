use task_629::*;

#[test]
fn edge_cases() {
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("hi|kit"),
            revision: 2
        }),
        8
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("|é|a"),
            revision: 3
        }),
        8
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from(""),
            revision: 4
        }),
        4
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("|"),
            revision: 5
        }),
        5
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("cat"),
            revision: 6
        }),
        9
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("||one|猫|ab"),
            revision: 7
        }),
        19
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("aa|bb"),
            revision: 8
        }),
        13
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("ok|🐈"),
            revision: 9
        }),
        16
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("header\nleft|right\r\n"),
            revision: 10
        }),
        20
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("header\n\n"),
            revision: 11
        }),
        11
    );
    assert_eq!(
        closing_marks_removed_pipe(&Record {
            label: String::from("猫|x\nend|a"),
            revision: 12
        }),
        17
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_629::TRACK_PROOF, 0x52540000 | 629);
}
