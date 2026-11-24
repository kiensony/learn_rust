use task_2472::*;

#[test]
fn edge_cases() {
    assert_eq!(strided_raw_reads_total(&[9, 2, 2, 8, 0, 5]), 11);
    assert_eq!(strided_raw_reads_total(&[0, 0, 0]), 0);
    assert_eq!(strided_raw_reads_total(&[255, 1, 255, 2]), 510);
    assert_eq!(
        strided_raw_reads_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        41
    );
    assert_eq!(strided_raw_reads_total(&[17, 17, 1, 0, 2]), 20);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2472::TRACK_PROOF, 0x52540000 | 2472);
}
