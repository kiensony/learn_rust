use task_1389::*;

#[test]
fn edge_cases() {
    assert_eq!(
        first_frame_byte_longest_run(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        1
    );
    assert_eq!(first_frame_byte_longest_run(&[3, 1]), 0);
    assert_eq!(first_frame_byte_longest_run(&[1, 0, 0, 1, 255]), 2);
    assert_eq!(first_frame_byte_longest_run(&[2, 3, 1, 2, 3, 1, 1, 2]), 2);
    assert_eq!(first_frame_byte_longest_run(&[255, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1389::TRACK_PROOF, 0x52540000 | 1389);
}
