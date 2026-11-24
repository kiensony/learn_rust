use task_2380::*;

#[test]
fn edge_cases() {
    assert_eq!(
        cumulative_occupancy_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 4u32, 4u32, 3u32, 2u32, 1u32]
    );
    assert_eq!(
        cumulative_occupancy_reverse(&[0, 0, 0]),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        cumulative_occupancy_reverse(&[255, 1, 255, 2]),
        vec![4u32, 3u32, 2u32, 1u32]
    );
    assert_eq!(
        cumulative_occupancy_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![9u32, 8u32, 7u32, 6u32, 6u32, 5u32, 4u32, 3u32, 2u32, 1u32]
    );
    assert_eq!(
        cumulative_occupancy_reverse(&[17, 17, 1, 0, 2]),
        vec![4u32, 3u32, 3u32, 2u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2380::TRACK_PROOF, 0x52540000 | 2380);
}
