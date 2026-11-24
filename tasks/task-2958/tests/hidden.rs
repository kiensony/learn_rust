use task_2958::*;

#[test]
fn edge_cases() {
    assert_eq!(journal_replay_steps(&[9, 2, 2, 8, 0, 5]), vec![7u32, 3u32]);
    assert_eq!(journal_replay_steps(&[0, 0, 0]), vec![]);
    assert_eq!(journal_replay_steps(&[255, 1, 255, 2]), vec![254u32]);
    assert_eq!(
        journal_replay_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 20u32]
    );
    assert_eq!(journal_replay_steps(&[17, 17, 1, 0, 2]), vec![0u32, 15u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2958::TRACK_PROOF, 0x52540000 | 2958);
}
