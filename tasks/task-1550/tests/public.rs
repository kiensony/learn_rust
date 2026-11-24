use task_1550::*;

#[test]
fn public_examples() {
    assert_eq!(
        packet_reversal_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[192, 96, 144, 48, 168, 240, 0, 88, 16]
        ),
        Some(9)
    );
    assert_eq!(packet_reversal_replay_mismatch(&[], &[]), None);
}
