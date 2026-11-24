use task_1957::*;

#[test]
fn edge_cases() {
    assert_eq!(
        atomic_running_total_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 9u32, 11u32, 13u32, 21u32]
    );
    assert_eq!(atomic_running_total_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        atomic_running_total_distinct(&[255, 1, 255, 2]),
        vec![0u32, 255u32, 256u32, 511u32]
    );
    assert_eq!(
        atomic_running_total_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 3u32, 9u32, 18u32, 30u32, 51u32, 66u32, 92u32, 100u32]
    );
    assert_eq!(
        atomic_running_total_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 17u32, 34u32, 35u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1957::TRACK_PROOF, 0x52540000 | 1957);
}
