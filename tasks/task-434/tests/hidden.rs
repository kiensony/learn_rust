use task_434::*;

#[test]
fn edge_cases() {
    assert_eq!(append_reading(vec![], 8), (vec![8], None));
    assert_eq!(append_reading(vec![-3], -6), (vec![-3, -6], Some(-3)));
    assert_eq!(append_reading(vec![0, 0], 0), (vec![0, 0, 0], Some(0)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_434::TRACK_PROOF, 0x52540000 | 434);
}
