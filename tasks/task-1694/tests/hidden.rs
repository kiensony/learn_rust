use task_1694::*;

#[test]
fn edge_cases() {
    assert_eq!(
        release_line_peaks("1.0.0 1.2.0 2.0.0"),
        vec![256u32, 258u32, 512u32]
    );
    assert_eq!(release_line_peaks("1 3.1.8 bad 3.1.8"), vec![769u32]);
    assert_eq!(
        release_line_peaks("0.255.3 1.0.9 256.1.1"),
        vec![255u32, 256u32]
    );
    assert_eq!(release_line_peaks("12.4.8 2.9.3 12.4.8"), vec![3076u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1694::TRACK_PROOF, 0x52540000 | 1694);
}
