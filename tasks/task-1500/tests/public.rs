use task_1500::*;

#[test]
fn public_examples() {
    assert_eq!(
        status_detail_replay_mismatch(" 302 204 503 400 201 ", &[2, 4, 3, 0]),
        Some(4)
    );
    assert_eq!(status_detail_replay_mismatch("", &[]), None);
}
