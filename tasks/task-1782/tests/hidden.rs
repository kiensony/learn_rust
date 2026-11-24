use task_1782::*;

#[test]
fn edge_cases() {
    assert_eq!(token_multiplier_total(&[9, 2, 2, 8, 0, 5]), 104);
    assert_eq!(token_multiplier_total(&[0, 0, 0]), 0);
    assert_eq!(token_multiplier_total(&[255, 1, 255, 2]), 2052);
    assert_eq!(
        token_multiplier_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        524
    );
    assert_eq!(token_multiplier_total(&[17, 17, 1, 0, 2]), 148);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1782::TRACK_PROOF, 0x52540000 | 1782);
}
