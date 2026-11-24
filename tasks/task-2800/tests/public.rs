use task_2800::*;

#[test]
fn public_examples() {
    assert_eq!(
        final_settings_replay_mismatch("a=0;b=1;b=2;z=255", &[0, 2]),
        Some(2)
    );
    assert_eq!(final_settings_replay_mismatch("", &[]), None);
}
