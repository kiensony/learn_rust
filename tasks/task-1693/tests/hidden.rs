use task_1693::*;

#[test]
fn edge_cases() {
    assert_eq!(
        release_line_bounds("1.0.0 1.2.0 2.0.0"),
        Some((256u32, 512u32))
    );
    assert_eq!(
        release_line_bounds("1 3.1.8 bad 3.1.8"),
        Some((769u32, 769u32))
    );
    assert_eq!(
        release_line_bounds("0.255.3 1.0.9 256.1.1"),
        Some((255u32, 256u32))
    );
    assert_eq!(
        release_line_bounds("12.4.8 2.9.3 12.4.8"),
        Some((521u32, 3076u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1693::TRACK_PROOF, 0x52540000 | 1693);
}
