use task_1762::*;

#[test]
fn edge_cases() {
    assert_eq!(generated_offset_total(&[9, 2, 2, 8, 0, 5]), 68);
    assert_eq!(generated_offset_total(&[0, 0, 0]), 21);
    assert_eq!(generated_offset_total(&[255, 1, 255, 2]), 541);
    assert_eq!(
        generated_offset_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        201
    );
    assert_eq!(generated_offset_total(&[17, 17, 1, 0, 2]), 72);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1762::TRACK_PROOF, 0x52540000 | 1762);
}
