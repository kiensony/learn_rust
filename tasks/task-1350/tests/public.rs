use task_1350::*;

#[test]
fn public_examples() {
    assert_eq!(
        packed_booleans_replay_mismatch("4,9,2,2,0", &[1, 2, 1, 1]),
        Some(4)
    );
    assert_eq!(packed_booleans_replay_mismatch("", &[]), None);
}
