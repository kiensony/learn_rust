use task_2601::*;

#[test]
fn edge_cases() {
    assert_eq!(
        embedded_checksum_trace(&[9, 2, 2, 8, 0, 5]),
        vec![132u32, 1u32, 1u32, 4u32, 0u32, 130u32]
    );
    assert_eq!(embedded_checksum_trace(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        embedded_checksum_trace(&[255, 1, 255, 2]),
        vec![255u32, 128u32, 255u32, 1u32]
    );
    assert_eq!(
        embedded_checksum_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            129u32, 3u32, 132u32, 6u32, 138u32, 135u32, 0u32, 13u32, 4u32, 143u32
        ]
    );
    assert_eq!(
        embedded_checksum_trace(&[17, 17, 1, 0, 2]),
        vec![136u32, 136u32, 128u32, 0u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2601::TRACK_PROOF, 0x52540000 | 2601);
}
