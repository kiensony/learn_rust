use task_1379::*;

#[test]
fn edge_cases() {
    assert_eq!(
        frame_checksum_longest_run(&[2, 7, 3, 1, 4, 0, 3, 9, 9, 2]),
        2
    );
    assert_eq!(frame_checksum_longest_run(&[3, 1]), 0);
    assert_eq!(frame_checksum_longest_run(&[1, 0, 0, 1, 255]), 2);
    assert_eq!(frame_checksum_longest_run(&[2, 3, 1, 2, 3, 1, 1, 2]), 3);
    assert_eq!(frame_checksum_longest_run(&[255, 0]), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1379::TRACK_PROOF, 0x52540000 | 1379);
}
