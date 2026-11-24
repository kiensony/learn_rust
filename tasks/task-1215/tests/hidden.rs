use task_1215::*;

#[test]
fn edge_cases() {
    assert_eq!(read_chunks_changes(&[9, 2, 2, 8, 0, 5]), vec![3u32]);
    assert_eq!(read_chunks_changes(&[0, 0, 0]), vec![3u32]);
    assert_eq!(read_chunks_changes(&[255, 1, 255, 2]), vec![3u32, 1u32]);
    assert_eq!(
        read_chunks_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![3u32, 1u32]
    );
    assert_eq!(read_chunks_changes(&[17, 17, 1, 0, 2]), vec![3u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1215::TRACK_PROOF, 0x52540000 | 1215);
}
