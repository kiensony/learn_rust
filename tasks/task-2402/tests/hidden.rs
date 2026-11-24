use task_2402::*;

#[test]
fn edge_cases() {
    assert_eq!(header_alignment_total(&[9, 2, 2, 8, 0, 5]), 28);
    assert_eq!(header_alignment_total(&[0, 0, 0]), 0);
    assert_eq!(header_alignment_total(&[255, 1, 255, 2]), 516);
    assert_eq!(
        header_alignment_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        136
    );
    assert_eq!(header_alignment_total(&[17, 17, 1, 0, 2]), 40);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2402::TRACK_PROOF, 0x52540000 | 2402);
}
