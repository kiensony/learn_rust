use task_632::*;

#[test]
fn edge_cases() {
    assert_eq!(
        border(Panel {
            width: 5,
            height: 3
        }),
        16
    );
    assert_eq!(
        border(Panel {
            width: 0,
            height: 5
        }),
        10
    );
    assert_eq!(
        border(Panel {
            width: 65535,
            height: 65535
        }),
        262140
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_632::TRACK_PROOF, 0x52540000 | 632);
}
