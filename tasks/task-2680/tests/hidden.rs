use task_2680::*;

#[test]
fn edge_cases() {
    assert_eq!(
        lending_checksum_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![5u32, 1u32]
    );
    assert_eq!(lending_checksum_reverse(&[0, 0, 0]), vec![0u32]);
    assert_eq!(lending_checksum_reverse(&[255, 1, 255, 2]), vec![3u32]);
    assert_eq!(
        lending_checksum_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![23u32, 0u32, 0u32]
    );
    assert_eq!(
        lending_checksum_reverse(&[17, 17, 1, 0, 2]),
        vec![2u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2680::TRACK_PROOF, 0x52540000 | 2680);
}
