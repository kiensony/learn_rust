use task_186::*;

#[test]
fn edge_cases() {
    assert_eq!(fare(0), 0);
    assert_eq!(fare(5), 2);
    assert_eq!(fare(18), 5);
    assert_eq!(fare(255), 5);
}

#[test]
fn submission_marker() {
    assert_eq!(task_186::TRACK_PROOF, 0x52540000 | 186);
}
