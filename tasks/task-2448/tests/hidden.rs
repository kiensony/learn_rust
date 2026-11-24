use task_2448::*;

#[test]
fn edge_cases() {
    assert_eq!(wide_alignment_rolling_load(&[4], 1), vec![8u64]);
    assert_eq!(
        wide_alignment_rolling_load(&[9, 2, 2, 8, 0, 5], 2),
        vec![24u64, 16u64, 16u64, 8u64, 8u64]
    );
    assert_eq!(wide_alignment_rolling_load(&[0, 0, 0], 0), vec![]);
    assert_eq!(wide_alignment_rolling_load(&[255, 1, 255, 2], 100), vec![]);
    assert_eq!(
        wide_alignment_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![32u64, 40u64, 56u64, 56u64, 40u64, 48u64, 40u64, 72u64]
    );
    assert_eq!(
        wide_alignment_rolling_load(&[17, 17, 1, 0, 2], 1),
        vec![24u64, 24u64, 8u64, 0u64, 8u64]
    );
    assert_eq!(
        wide_alignment_rolling_load(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 2),
        vec![
            16u64, 24u64, 32u64, 40u64, 40u64, 16u64, 32u64, 40u64, 40u64
        ]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2448::TRACK_PROOF, 0x52540000 | 2448);
}
