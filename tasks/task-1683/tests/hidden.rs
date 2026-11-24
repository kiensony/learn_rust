use task_1683::*;

#[test]
fn edge_cases() {
    assert_eq!(
        preview_version_bounds("1.0.0 1.2.0 2.0.0"),
        Some((0u32, 0u32))
    );
    assert_eq!(
        preview_version_bounds("1 3.1.8 bad 3.1.8"),
        Some((0u32, 0u32))
    );
    assert_eq!(
        preview_version_bounds("0.255.3 1.0.9 256.1.1"),
        Some((0u32, 1u32))
    );
    assert_eq!(
        preview_version_bounds("12.4.8 2.9.3 12.4.8"),
        Some((0u32, 0u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1683::TRACK_PROOF, 0x52540000 | 1683);
}
