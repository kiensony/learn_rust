use task_1700::*;

#[test]
fn public_examples() {
    assert_eq!(
        release_line_replay_mismatch("1.0.0 1.2.0 2.0.0", &[256, 258]),
        Some(2)
    );
    assert_eq!(release_line_replay_mismatch("", &[]), None);
}
