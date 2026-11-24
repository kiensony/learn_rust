use task_2300::*;

#[test]
fn public_examples() {
    assert_eq!(
        profile_ranking_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[31, 26, 21, 15, 12, 9, 8, 6, 3]
        ),
        Some(9)
    );
    assert_eq!(profile_ranking_replay_mismatch(&[], &[]), None);
}
