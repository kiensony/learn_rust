use task_1111::*;

#[test]
fn edge_cases() {
    assert_eq!(sample_all(&[]), Vec::<i16>::new());
    assert_eq!(sample_all(&[&Constant(0), &Constant(0)]), vec![0, 0]);
    assert_eq!(sample_all(&[&Negate(-128)]), vec![128]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1111::TRACK_PROOF, 0x52540000 | 1111);
}
