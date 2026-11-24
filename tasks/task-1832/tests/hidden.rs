use task_1832::*;

#[test]
fn edge_cases() {
    assert_eq!(shard_occupancy_total(&[9, 2, 2, 8, 0, 5]), 5);
    assert_eq!(shard_occupancy_total(&[0, 0, 0]), 0);
    assert_eq!(shard_occupancy_total(&[255, 1, 255, 2]), 4);
    assert_eq!(
        shard_occupancy_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        9
    );
    assert_eq!(shard_occupancy_total(&[17, 17, 1, 0, 2]), 4);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1832::TRACK_PROOF, 0x52540000 | 1832);
}
