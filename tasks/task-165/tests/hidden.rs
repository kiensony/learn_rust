use task_165::*;

#[test]
fn edge_cases() {
    assert_eq!(even_batch_2(0), false);
    assert_eq!(even_batch_2(1), false);
    assert_eq!(even_batch_2(1), false);
    assert_eq!(even_batch_2(2), true);
    assert_eq!(even_batch_2(3), false);
    assert_eq!(even_batch_2(65535), false);
    assert_eq!(even_batch_2(4), true);
    assert_eq!(even_batch_2(5), false);
    assert_eq!(even_batch_2(4), true);
    assert_eq!(even_batch_2(5), false);
}

#[test]
fn submission_marker() {
    assert_eq!(task_165::TRACK_PROOF, 0x52540000 | 165);
}
