use task_2392::*;

#[test]
fn edge_cases() {
    assert_eq!(last_change_age_total(&[9, 2, 2, 8, 0, 5]), 7);
    assert_eq!(last_change_age_total(&[0, 0, 0]), 6);
    assert_eq!(last_change_age_total(&[255, 1, 255, 2]), 4);
    assert_eq!(
        last_change_age_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        10
    );
    assert_eq!(last_change_age_total(&[17, 17, 1, 0, 2]), 6);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2392::TRACK_PROOF, 0x52540000 | 2392);
}
