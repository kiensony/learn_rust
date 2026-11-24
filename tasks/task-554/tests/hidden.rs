use task_554::*;

#[test]
fn edge_cases() {
    assert_eq!(puzzle_cells(""), (0, 0));
    assert_eq!(puzzle_cells("··"), (0, 2));
}

#[test]
fn submission_marker() {
    assert_eq!(task_554::TRACK_PROOF, 0x52540000 | 554);
}
