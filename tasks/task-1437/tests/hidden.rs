use task_1437::*;

#[test]
fn edge_cases() {
    assert_eq!(
        timer_windows_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 1u32, 2u32]
    );
    assert_eq!(timer_windows_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(timer_windows_distinct(&[255, 1, 255, 2]), vec![1u32, 44u32]);
    assert_eq!(
        timer_windows_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32]
    );
    assert_eq!(
        timer_windows_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1437::TRACK_PROOF, 0x52540000 | 1437);
}
