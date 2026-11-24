use task_2800::*;

#[test]
fn edge_cases() {
    assert_eq!(final_settings_replay_mismatch("a=1", &[1]), None);
    assert_eq!(
        final_settings_replay_mismatch("b=02;a=9;a=9;b=4", &[9, 4, 777]),
        Some(2)
    );
    assert_eq!(
        final_settings_replay_mismatch("missing;=4;c=256;a=x;;a=4", &[]),
        Some(0)
    );
    assert_eq!(
        final_settings_replay_mismatch("a=0;b=1;b=2;z=255", &[9999]),
        Some(0)
    );
    assert_eq!(
        final_settings_replay_mismatch("xx=3;z=0;LONG=12", &[0, 3, 12]),
        Some(0)
    );
    assert_eq!(
        final_settings_replay_mismatch("a=0;b=1;b=2;z=255", &[0, 2, 255, 1]),
        Some(3)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2800::TRACK_PROOF, 0x52540000 | 2800);
}
