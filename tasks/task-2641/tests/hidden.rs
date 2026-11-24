use task_2641::*;

#[test]
fn edge_cases() {
    assert_eq!(
        embedded_nibble_swap_trace(&[9, 2, 2, 8, 0, 5]),
        vec![144u32, 32u32, 32u32, 128u32, 0u32, 80u32]
    );
    assert_eq!(
        embedded_nibble_swap_trace(&[0, 0, 0]),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        embedded_nibble_swap_trace(&[255, 1, 255, 2]),
        vec![255u32, 16u32, 255u32, 32u32]
    );
    assert_eq!(
        embedded_nibble_swap_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            48u32, 96u32, 144u32, 192u32, 81u32, 240u32, 0u32, 161u32, 128u32, 241u32
        ]
    );
    assert_eq!(
        embedded_nibble_swap_trace(&[17, 17, 1, 0, 2]),
        vec![17u32, 17u32, 16u32, 0u32, 32u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2641::TRACK_PROOF, 0x52540000 | 2641);
}
