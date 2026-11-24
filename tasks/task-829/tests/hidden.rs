use task_829::*;

#[test]
fn edge_cases() {
    assert_eq!(combine(Ok(1), Err(Field::Right)), Err(Field::Right));
    assert_eq!(combine(Err(Field::Right), Ok(2)), Err(Field::Right));
    assert_eq!(combine(Ok(255), Ok(255)), Ok(510));
}

#[test]
fn submission_marker() {
    assert_eq!(task_829::TRACK_PROOF, 0x52540000 | 829);
}
