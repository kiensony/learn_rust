use task_2650::*;

#[test]
fn edge_cases() {
    assert_eq!(embedded_nibble_swap_replay_mismatch(&[4], &[64]), None);
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[144, 32, 32, 128, 0, 80, 777]),
        Some(6)
    );
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(&[0, 0, 0], &[0, 0]),
        Some(2)
    );
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[241, 128, 161, 0, 240, 81, 192, 144, 96, 48]
        ),
        Some(0)
    );
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(&[17, 17, 1, 0, 2], &[17, 17, 16, 0, 32]),
        None
    );
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[48, 96, 144, 192, 81, 240, 0, 161, 128, 241, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2650::TRACK_PROOF, 0x52540000 | 2650);
}
