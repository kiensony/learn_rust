use task_2642::*;

#[test]
fn edge_cases() {
    assert_eq!(embedded_nibble_swap_total(&[9, 2, 2, 8, 0, 5]), 416);
    assert_eq!(embedded_nibble_swap_total(&[0, 0, 0]), 0);
    assert_eq!(embedded_nibble_swap_total(&[255, 1, 255, 2]), 558);
    assert_eq!(
        embedded_nibble_swap_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        1331
    );
    assert_eq!(embedded_nibble_swap_total(&[17, 17, 1, 0, 2]), 82);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2642::TRACK_PROOF, 0x52540000 | 2642);
}
