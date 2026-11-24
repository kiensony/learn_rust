use task_1500::*;

#[test]
fn edge_cases() {
    assert_eq!(status_detail_replay_mismatch("200", &[0]), None);
    assert_eq!(
        status_detail_replay_mismatch("200 503 404 200", &[0, 3, 4, 0, 777]),
        Some(4)
    );
    assert_eq!(
        status_detail_replay_mismatch("xx 20 099 201 600", &[]),
        Some(0)
    );
    assert_eq!(
        status_detail_replay_mismatch("500 500 204 301 404", &[9999]),
        Some(0)
    );
    assert_eq!(
        status_detail_replay_mismatch(" 302 204 503 400 201 ", &[1, 0, 3, 4, 2]),
        Some(0)
    );
    assert_eq!(
        status_detail_replay_mismatch(" 302 204 503 400 201 ", &[2, 4, 3, 0, 1, 1]),
        Some(5)
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1500::TRACK_PROOF, 0x52540000 | 1500);
}
