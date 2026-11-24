use task_1600::*;

#[test]
fn transfer_case() {
    assert_eq!(
        feature_pages_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ]
        ),
        if cfg!(feature = "wide") {
            Some(0)
        } else {
            None
        }
    );
}
