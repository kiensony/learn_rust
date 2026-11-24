use task_2832::*;

#[test]
fn edge_cases() {
    assert_eq!(service_expansion_total(&[9, 2, 2, 8, 0, 5]), 52);
    assert_eq!(service_expansion_total(&[0, 0, 0]), 0);
    assert_eq!(service_expansion_total(&[255, 1, 255, 2]), 1026);
    assert_eq!(
        service_expansion_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        262
    );
    assert_eq!(service_expansion_total(&[17, 17, 1, 0, 2]), 74);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2832::TRACK_PROOF, 0x52540000 | 2832);
}
