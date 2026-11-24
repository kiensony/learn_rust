use task_2372::*;

#[test]
fn edge_cases() {
    assert_eq!(cumulative_occupancy_total(&[9, 2, 2, 8, 0, 5]), 19);
    assert_eq!(cumulative_occupancy_total(&[0, 0, 0]), 0);
    assert_eq!(cumulative_occupancy_total(&[255, 1, 255, 2]), 10);
    assert_eq!(
        cumulative_occupancy_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        51
    );
    assert_eq!(cumulative_occupancy_total(&[17, 17, 1, 0, 2]), 13);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2372::TRACK_PROOF, 0x52540000 | 2372);
}
