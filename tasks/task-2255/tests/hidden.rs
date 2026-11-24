use task_2255::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cpu_sample_counts_changes(&[9, 2, 2, 8, 0, 5]),
        vec![1u32, 2u32, 1u32]
    );
    assert_eq!(cpu_sample_counts_changes(&[0, 0, 0]), vec![3u32]);
    assert_eq!(
        cpu_sample_counts_changes(&[255, 1, 255, 2]),
        vec![1u32, 2u32]
    );
    assert_eq!(
        cpu_sample_counts_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1u32]
    );
    assert_eq!(
        cpu_sample_counts_changes(&[17, 17, 1, 0, 2]),
        vec![1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2255::TRACK_PROOF, 0x52540000 | 2255);
}
