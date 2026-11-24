use task_1700::*;

#[test]
fn transfer_case() {
    assert_eq!(
        release_line_replay_mismatch(
            "1.0.0 1.2.0 2.0.0 1.0.0 1.2.0 2.0.0",
            &[256, 258, 512, 256, 258, 512]
        ),
        None
    );
}
