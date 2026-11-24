use task_2302::*;

#[test]
fn edge_cases() {
    assert_eq!(warmup_discard_total(&[9, 2, 2, 8, 0, 5]), 15);
    assert_eq!(warmup_discard_total(&[0, 0, 0]), 0);
    assert_eq!(warmup_discard_total(&[255, 1, 255, 2]), 257);
    assert_eq!(
        warmup_discard_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        122
    );
    assert_eq!(warmup_discard_total(&[17, 17, 1, 0, 2]), 3);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2302::TRACK_PROOF, 0x52540000 | 2302);
}
