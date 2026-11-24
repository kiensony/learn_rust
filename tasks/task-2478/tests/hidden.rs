use task_2478::*;

#[test]
fn edge_cases() {
    assert_eq!(
        strided_raw_reads_steps(&[9, 2, 2, 8, 0, 5]),
        vec![7u32, 2u32]
    );
    assert_eq!(strided_raw_reads_steps(&[0, 0, 0]), vec![0u32]);
    assert_eq!(strided_raw_reads_steps(&[255, 1, 255, 2]), vec![0u32]);
    assert_eq!(
        strided_raw_reads_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![6u32, 12u32, 21u32, 8u32]
    );
    assert_eq!(
        strided_raw_reads_steps(&[17, 17, 1, 0, 2]),
        vec![16u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2478::TRACK_PROOF, 0x52540000 | 2478);
}
