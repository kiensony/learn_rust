use task_1350::*;

#[test]
fn transfer_case() {
    assert_eq!(
        packed_booleans_replay_mismatch("4,9,2,2,0 4,9,2,2,0", &[1, 2, 1, 1, 2, 1, 1, 0]),
        None
    );
}
