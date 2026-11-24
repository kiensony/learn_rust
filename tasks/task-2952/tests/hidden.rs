use task_2952::*;

#[test]
fn edge_cases() {
    assert_eq!(journal_replay_total(&[9, 2, 2, 8, 0, 5]), 16);
    assert_eq!(journal_replay_total(&[0, 0, 0]), 0);
    assert_eq!(journal_replay_total(&[255, 1, 255, 2]), 256);
    assert_eq!(
        journal_replay_total(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        35
    );
    assert_eq!(journal_replay_total(&[17, 17, 1, 0, 2]), 36);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2952::TRACK_PROOF, 0x52540000 | 2952);
}
