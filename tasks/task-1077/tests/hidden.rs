use task_1077::*;

#[test]
fn edge_cases() {
    assert_eq!(consume_first(std::iter::empty::<u8>()), None);
    assert_eq!(consume_first(std::iter::once(false)), Some(false));
    assert_eq!(consume_first((9..11).rev()), Some(10));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1077::TRACK_PROOF, 0x52540000 | 1077);
}
