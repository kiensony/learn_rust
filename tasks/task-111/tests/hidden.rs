use task_111::*;

#[test]
fn edge_cases() {
    assert_eq!(change(0, 0), 0);
    assert_eq!(change(-128, 127), 255);
    assert_eq!(change(127, -128), -255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_111::TRACK_PROOF, 0x52540000 | 111);
}
