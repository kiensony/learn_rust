use task_1800::*;

#[test]
fn transfer_case() {
    assert_eq!(
        token_offset_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[
                8, 11, 14, 17, 26, 20, 5, 31, 13, 36, 36, 13, 31, 5, 20, 26, 17, 14, 11, 8
            ]
        ),
        None
    );
}
