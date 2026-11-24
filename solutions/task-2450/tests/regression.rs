use task_2450::*;

#[test]
fn transfer_case() {
    assert_eq!(
        page_alignment_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[
                32, 32, 32, 32, 32, 32, 0, 32, 32, 32, 32, 32, 32, 0, 32, 32, 32, 32, 32, 32
            ]
        ),
        None
    );
}
