use task_1700::*;

#[test]
fn edge_cases() {
    assert_eq!(release_line_replay_mismatch("0.0.1", &[0]), None);
    assert_eq!(
        release_line_replay_mismatch("1.0.0 1.2.0 2.0.0", &[256, 258, 512, 777]),
        Some(3)
    );
    assert_eq!(
        release_line_replay_mismatch("1 3.1.8 bad 3.1.8", &[769]),
        Some(1)
    );
    assert_eq!(
        release_line_replay_mismatch("0.255.3 1.0.9 256.1.1", &[9999]),
        Some(0)
    );
    assert_eq!(
        release_line_replay_mismatch("12.4.8 2.9.3 12.4.8", &[3076, 521, 3076]),
        None
    );
    assert_eq!(
        release_line_replay_mismatch("1.0.0 1.2.0 2.0.0", &[256, 258, 512, 1]),
        Some(3)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1700::TRACK_PROOF, 0x52540000 | 1700);
}
