use task_2691::*;

#[test]
fn edge_cases() {
    assert_eq!(universal_slice_width_trace(&[9, 2, 2, 8, 0, 5]), vec![9u32]);
    assert_eq!(universal_slice_width_trace(&[0, 0, 0]), vec![0u32]);
    assert_eq!(universal_slice_width_trace(&[255, 1, 255, 2]), vec![254u32]);
    assert_eq!(
        universal_slice_width_trace(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![18u32, 31u32]
    );
    assert_eq!(universal_slice_width_trace(&[17, 17, 1, 0, 2]), vec![17u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2691::TRACK_PROOF, 0x52540000 | 2691);
}
