use task_813::*;

#[test]
fn edge_cases() {
    assert_eq!(reading(""), Err(ReadingError::Blank));
    assert_eq!(reading("x"), Err(ReadingError::Number));
    assert_eq!(reading("70000"), Err(ReadingError::Number));
    assert_eq!(reading("255"), Ok(255));
}

#[test]
fn submission_marker() {
    assert_eq!(task_813::TRACK_PROOF, 0x52540000 | 813);
}
