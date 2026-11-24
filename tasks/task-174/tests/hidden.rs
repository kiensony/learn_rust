use task_174::*;

#[test]
fn edge_cases() {
    assert_eq!(even_batch_3(13, 3), true);
    assert_eq!(even_batch_3(0, 6), true);
    assert_eq!(even_batch_3(1, 9), true);
    assert_eq!(even_batch_3(2, 12), true);
    assert_eq!(even_batch_3(3, 15), true);
    assert_eq!(even_batch_3(65535, 4), false);
    assert_eq!(even_batch_3(5, 7), true);
    assert_eq!(even_batch_3(6, 10), true);
    assert_eq!(even_batch_3(6, 13), false);
    assert_eq!(even_batch_3(7, 16), false);
    assert_eq!(even_batch_3(65535, 1), false);
    assert_eq!(even_batch_3(0, 0), false);
    assert_eq!(even_batch_3(1, 0), false);
    assert_eq!(even_batch_3(2, 0), false);
    assert_eq!(even_batch_3(3, 0), false);
    assert_eq!(even_batch_3(5, 0), false);
    assert_eq!(even_batch_3(6, 0), true);
    assert_eq!(even_batch_3(7, 0), false);
    assert_eq!(even_batch_3(20, 0), true);
    assert_eq!(even_batch_3(100, 0), true);
}

#[test]
fn submission_marker() {
    assert_eq!(task_174::TRACK_PROOF, 0x52540000 | 174);
}
