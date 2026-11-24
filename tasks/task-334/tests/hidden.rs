use task_334::*;

#[test]
fn edge_cases() {
    assert_eq!(both_points((0, 0)), [(0, 0), (0, 0)]);
    assert_eq!(
        both_points((-32768, 32767)),
        [(-32768, 32767), (-32768, 32767)]
    );
    assert_eq!(both_points((7, 7)), [(7, 7), (7, 7)]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_334::TRACK_PROOF, 0x52540000 | 334);
}
