use task_2990::*;

#[test]
fn edge_cases() {
    assert_eq!(service_wait_reverse(&[9, 2, 2, 8, 0, 5]), vec![4u32]);
    assert_eq!(service_wait_reverse(&[0, 0, 0]), vec![]);
    assert_eq!(service_wait_reverse(&[255, 1, 255, 2]), vec![]);
    assert_eq!(
        service_wait_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![6u32]
    );
    assert_eq!(service_wait_reverse(&[17, 17, 1, 0, 2]), vec![3u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2990::TRACK_PROOF, 0x52540000 | 2990);
}
