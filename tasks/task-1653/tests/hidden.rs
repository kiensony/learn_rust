use task_1653::*;

#[test]
fn edge_cases() {
    assert_eq!(
        major_version_bounds("1.0.0 1.2.0 2.0.0"),
        Some((1u32, 2u32))
    );
    assert_eq!(
        major_version_bounds("1 3.1.8 bad 3.1.8"),
        Some((3u32, 3u32))
    );
    assert_eq!(
        major_version_bounds("0.255.3 1.0.9 256.1.1"),
        Some((0u32, 1u32))
    );
    assert_eq!(
        major_version_bounds("12.4.8 2.9.3 12.4.8"),
        Some((2u32, 12u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1653::TRACK_PROOF, 0x52540000 | 1653);
}
