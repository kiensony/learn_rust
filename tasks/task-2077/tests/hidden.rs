use task_2077::*;

#[test]
fn edge_cases() {
    assert_eq!(cancel_reservation(0, 0), vec![0, 0]);
    assert_eq!(cancel_reservation(255, 0), vec![255, 0]);
    assert_eq!(cancel_reservation(3, 255), vec![3, 255]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2077::TRACK_PROOF, 0x52540000 | 2077);
}
