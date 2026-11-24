use task_849::*;

#[test]
fn edge_cases() {
    assert_eq!(dose_total(&[]), Ok(0));
    assert_eq!(
        dose_total(&[-128]),
        Err(DoseError {
            index: 0,
            value: -128
        })
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_849::TRACK_PROOF, 0x52540000 | 849);
}
