use task_2317::*;

#[test]
fn edge_cases() {
    assert_eq!(
        paired_speedup_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 7u32]
    );
    assert_eq!(paired_speedup_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        paired_speedup_distinct(&[255, 1, 255, 2]),
        vec![253u32, 254u32]
    );
    assert_eq!(
        paired_speedup_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![0u32, 6u32]
    );
    assert_eq!(
        paired_speedup_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2317::TRACK_PROOF, 0x52540000 | 2317);
}
