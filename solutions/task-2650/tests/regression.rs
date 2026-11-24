use task_2650::*;

#[test]
fn transfer_case() {
    assert_eq!(
        embedded_nibble_swap_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[
                48, 96, 144, 192, 81, 240, 0, 161, 128, 241, 241, 128, 161, 0, 240, 81, 192, 144,
                96, 48
            ]
        ),
        None
    );
}
