use task_2322::*;

#[test]
fn edge_cases() {
    assert_eq!(batch_medians_total(&[9, 2, 2, 8, 0, 5]), 7);
    assert_eq!(batch_medians_total(&[0, 0, 0]), 0);
    assert_eq!(batch_medians_total(&[255, 1, 255, 2]), 257);
    assert_eq!(
        batch_medians_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        60
    );
    assert_eq!(batch_medians_total(&[17, 17, 1, 0, 2]), 17);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2322::TRACK_PROOF, 0x52540000 | 2322);
}
