use task_1860::*;

#[test]
fn edge_cases() {
    assert_eq!(
        shared_read_handles_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![2u32, 1u32, 1u32, 3u32, 3u32, 2u32]
    );
    assert_eq!(
        shared_read_handles_reverse(&[0, 0, 0]),
        vec![1u32, 1u32, 1u32]
    );
    assert_eq!(
        shared_read_handles_reverse(&[255, 1, 255, 2]),
        vec![3u32, 4u32, 2u32, 4u32]
    );
    assert_eq!(
        shared_read_handles_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![4u32, 1u32, 3u32, 1u32, 4u32, 2u32, 1u32, 2u32, 3u32, 4u32]
    );
    assert_eq!(
        shared_read_handles_reverse(&[17, 17, 1, 0, 2]),
        vec![3u32, 1u32, 2u32, 2u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1860::TRACK_PROOF, 0x52540000 | 1860);
}
