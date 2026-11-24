use task_184::*;

#[test]
fn edge_cases() {
    assert_eq!(even_batch_5(0, 2), false);
    assert_eq!(even_batch_5(1, 0), false);
    assert_eq!(even_batch_5(4, 1), false);
    assert_eq!(even_batch_5(5, 2), false);
    assert_eq!(even_batch_5(6, 0), true);
    assert_eq!(even_batch_5(65535, 1), true);
    assert_eq!(even_batch_5(7, 2), false);
    assert_eq!(even_batch_5(8, 0), true);
    assert_eq!(even_batch_5(10, 1), false);
    assert_eq!(even_batch_5(11, 2), false);
    assert_eq!(even_batch_5(0, 9), false);
    assert_eq!(even_batch_5(100, 0), true);
    assert_eq!(even_batch_5(100, 1), false);
    assert_eq!(even_batch_5(0, 0), false);
    assert_eq!(even_batch_5(5, 0), false);
    assert_eq!(even_batch_5(7, 0), false);
    assert_eq!(even_batch_5(10, 0), true);
    assert_eq!(even_batch_5(11, 0), false);
    assert_eq!(even_batch_5(20, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_184::TRACK_PROOF, 0x52540000 | 184);
}
