use task_2607::*;

#[test]
fn edge_cases() {
    assert_eq!(
        embedded_checksum_distinct(&[9, 2, 2, 8, 0, 5]),
        vec![0u32, 1u32, 4u32, 130u32, 132u32]
    );
    assert_eq!(embedded_checksum_distinct(&[0, 0, 0]), vec![0u32]);
    assert_eq!(
        embedded_checksum_distinct(&[255, 1, 255, 2]),
        vec![1u32, 128u32, 255u32]
    );
    assert_eq!(
        embedded_checksum_distinct(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            0u32, 3u32, 4u32, 6u32, 13u32, 129u32, 132u32, 135u32, 138u32, 143u32
        ]
    );
    assert_eq!(
        embedded_checksum_distinct(&[17, 17, 1, 0, 2]),
        vec![0u32, 1u32, 128u32, 136u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2607::TRACK_PROOF, 0x52540000 | 2607);
}
