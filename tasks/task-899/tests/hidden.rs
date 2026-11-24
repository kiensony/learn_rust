use task_899::*;

#[test]
fn edge_cases() {
    assert_eq!(wire_value(Mode::Reserved(0)), 0);
    assert_eq!(wire_value(Mode::Reserved(255)), 255);
}

#[test]
fn submission_marker() {
    assert_eq!(task_899::TRACK_PROOF, 0x52540000 | 899);
}
