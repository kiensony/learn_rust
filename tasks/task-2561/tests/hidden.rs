use task_2561::*;

#[test]
fn edge_cases() {
    assert_eq!(
        odd_element_proof_trace(&[9, 2, 2, 8, 0, 5]),
        vec![3u32, 9u32, 6u32]
    );
    assert_eq!(odd_element_proof_trace(&[0, 0, 0]), vec![1u32]);
    assert_eq!(odd_element_proof_trace(&[255, 1, 255, 2]), vec![2u32, 3u32]);
    assert_eq!(
        odd_element_proof_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![7u32, 13u32, 16u32, 27u32, 32u32]
    );
    assert_eq!(
        odd_element_proof_trace(&[17, 17, 1, 0, 2]),
        vec![18u32, 1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2561::TRACK_PROOF, 0x52540000 | 2561);
}
