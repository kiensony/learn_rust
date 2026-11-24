use task_2352::*;

#[test]
fn edge_cases() {
    assert_eq!(single_pass_prefix_maximum_total(&[9, 2, 2, 8, 0, 5]), 54);
    assert_eq!(single_pass_prefix_maximum_total(&[0, 0, 0]), 0);
    assert_eq!(single_pass_prefix_maximum_total(&[255, 1, 255, 2]), 1020);
    assert_eq!(
        single_pass_prefix_maximum_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        176
    );
    assert_eq!(single_pass_prefix_maximum_total(&[17, 17, 1, 0, 2]), 85);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2352::TRACK_PROOF, 0x52540000 | 2352);
}
