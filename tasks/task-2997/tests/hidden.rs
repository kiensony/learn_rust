use task_2997::*;

#[test]
fn edge_cases() {
    assert_eq!(reconcile(&[(9, 4), (2, 8), (9, 3), (2, 1)]), Err(9));
    assert_eq!(reconcile(&[(0, 65535), (1, 65535)]), Ok(131070));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2997::TRACK_PROOF, 0x52540000 | 2997);
}
