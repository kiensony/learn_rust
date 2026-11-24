use task_2905::*;

#[test]
fn edge_cases() {
    assert_eq!(
        latency_increases_changes(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 6u32, 5u32]
    );
    assert_eq!(latency_increases_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(latency_increases_changes(&[255, 1, 255, 2]), vec![0u32]);
    assert_eq!(
        latency_increases_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 0u32, 26u32, 23u32]
    );
    assert_eq!(latency_increases_changes(&[17, 17, 1, 0, 2]), vec![0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2905::TRACK_PROOF, 0x52540000 | 2905);
}
