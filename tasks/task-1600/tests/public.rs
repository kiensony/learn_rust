use task_1600::*;

#[test]
fn public_examples() {
    assert_eq!(
        feature_pages_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[3, 6, 9, 12, 21, 15, 0, 26, 8]
        ),
        if cfg!(feature = "wide") {
            Some(0)
        } else {
            Some(9)
        }
    );
    assert_eq!(feature_pages_replay_mismatch(&[], &[]), None);
}
