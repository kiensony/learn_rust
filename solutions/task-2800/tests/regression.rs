use task_2800::*;

#[test]
fn transfer_case() {
    assert_eq!(
        final_settings_replay_mismatch("a=0;b=1;b=2;z=255 a=0;b=1;b=2;z=255", &[0, 2, 255]),
        None
    );
}
