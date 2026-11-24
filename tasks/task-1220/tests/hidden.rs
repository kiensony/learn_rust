use task_1220::*;

#[test]
fn edge_cases() {
    assert_eq!(read_chunks_reverse(&[9, 2, 2, 8, 0, 5]), vec![3u32, 3u32]);
    assert_eq!(read_chunks_reverse(&[0, 0, 0]), vec![3u32]);
    assert_eq!(read_chunks_reverse(&[255, 1, 255, 2]), vec![1u32, 3u32]);
    assert_eq!(
        read_chunks_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![1u32, 3u32, 3u32, 3u32]
    );
    assert_eq!(read_chunks_reverse(&[17, 17, 1, 0, 2]), vec![2u32, 3u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1220::TRACK_PROOF, 0x52540000 | 1220);
}
