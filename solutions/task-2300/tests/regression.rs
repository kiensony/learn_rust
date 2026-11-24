use task_2300::*;

#[test]
fn transfer_case() {
    assert_eq!(
        profile_ranking_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[31, 26, 21, 15, 12, 9, 8, 6, 3, 0]
        ),
        None
    );
}
