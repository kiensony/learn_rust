use task_1350::*;

#[test]
fn edge_cases() {
    assert_eq!(packed_booleans_replay_mismatch("1", &[1]), None);
    assert_eq!(
        packed_booleans_replay_mismatch("4,9,2,2,0", &[1, 2, 1, 1, 0, 777]),
        Some(5)
    );
    assert_eq!(
        packed_booleans_replay_mismatch(" , 12,xyz,-1,,255,256,", &[2]),
        Some(1)
    );
    assert_eq!(packed_booleans_replay_mismatch("0,0,0", &[9999]), Some(0));
    assert_eq!(
        packed_booleans_replay_mismatch("12,3,12,4,5", &[2, 1, 2, 2, 2]),
        Some(1)
    );
    assert_eq!(
        packed_booleans_replay_mismatch("1, 02 ,3", &[1, 1, 2]),
        None
    );
    assert_eq!(
        packed_booleans_replay_mismatch("4,9,2,2,0", &[1, 2, 1, 1, 0, 1]),
        Some(5)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1350::TRACK_PROOF, 0x52540000 | 1350);
}
