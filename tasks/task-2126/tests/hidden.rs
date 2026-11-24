use task_2126::*;

#[test]
fn edge_cases() {
    assert_eq!(queue_rotation_descending_steps(&[9, 2, 2, 8, 0, 5]), 1);
    assert_eq!(queue_rotation_descending_steps(&[0, 0, 0]), 0);
    assert_eq!(queue_rotation_descending_steps(&[255, 1, 255, 2]), 1);
    assert_eq!(
        queue_rotation_descending_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        4
    );
    assert_eq!(queue_rotation_descending_steps(&[17, 17, 1, 0, 2]), 2);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2126::TRACK_PROOF, 0x52540000 | 2126);
}
