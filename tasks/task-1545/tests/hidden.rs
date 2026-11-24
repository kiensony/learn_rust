use task_1545::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packet_reversal_changes(&[9, 2, 2, 8, 0, 5]),
        vec![144u32, 64u32, 16u32, 0u32, 160u32]
    );
    assert_eq!(packet_reversal_changes(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        packet_reversal_changes(&[255, 1, 255, 2]),
        vec![255u32, 128u32, 255u32, 64u32]
    );
    assert_eq!(
        packet_reversal_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            192u32, 96u32, 144u32, 48u32, 168u32, 240u32, 0u32, 88u32, 16u32, 248u32
        ]
    );
    assert_eq!(
        packet_reversal_changes(&[17, 17, 1, 0, 2]),
        vec![136u32, 128u32, 0u32, 64u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1545::TRACK_PROOF, 0x52540000 | 1545);
}
