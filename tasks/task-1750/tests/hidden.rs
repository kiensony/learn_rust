use task_1750::*;

#[test]
fn edge_cases() {
    assert_eq!(rotate_macro_replay_mismatch(&[4], &[16]), None);
    assert_eq!(
        rotate_macro_replay_mismatch(&[9, 2, 2, 8, 0, 5], &[36, 8, 8, 32, 0, 20, 777]),
        Some(6)
    );
    assert_eq!(rotate_macro_replay_mismatch(&[0, 0, 0], &[0, 0]), Some(2));
    assert_eq!(
        rotate_macro_replay_mismatch(&[255, 1, 255, 2], &[9999]),
        Some(0)
    );
    assert_eq!(
        rotate_macro_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[124, 32, 104, 0, 60, 84, 48, 36, 24, 12]
        ),
        Some(0)
    );
    assert_eq!(
        rotate_macro_replay_mismatch(&[17, 17, 1, 0, 2], &[68, 68, 4, 0, 8]),
        None
    );
    assert_eq!(
        rotate_macro_replay_mismatch(
            &[3, 6, 9, 12, 21, 15, 0, 26, 8, 31],
            &[12, 24, 36, 48, 84, 60, 0, 104, 32, 124, 1]
        ),
        Some(10)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1750::TRACK_PROOF, 0x52540000 | 1750);
}
