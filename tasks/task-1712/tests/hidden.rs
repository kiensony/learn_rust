use task_1712::*;

#[test]
fn edge_cases() {
    assert_eq!(saturating_macro_total(&[9, 2, 2, 8, 0, 5]), 50);
    assert_eq!(saturating_macro_total(&[0, 0, 0]), 12);
    assert_eq!(saturating_macro_total(&[255, 1, 255, 2]), 521);
    assert_eq!(
        saturating_macro_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        171
    );
    assert_eq!(saturating_macro_total(&[17, 17, 1, 0, 2]), 57);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1712::TRACK_PROOF, 0x52540000 | 1712);
}
