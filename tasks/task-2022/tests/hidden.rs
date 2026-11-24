use task_2022::*;

#[test]
fn edge_cases() {
    assert_eq!(polled_groups_total(&[9, 2, 2, 8, 0, 5]), 5);
    assert_eq!(polled_groups_total(&[0, 0, 0]), 0);
    assert_eq!(polled_groups_total(&[255, 1, 255, 2]), 126);
    assert_eq!(
        polled_groups_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        29
    );
    assert_eq!(polled_groups_total(&[17, 17, 1, 0, 2]), 8);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2022::TRACK_PROOF, 0x52540000 | 2022);
}
