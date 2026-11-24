use task_2313::*;

#[test]
fn edge_cases() {
    assert_eq!(
        paired_speedup_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((0u32, 7u32))
    );
    assert_eq!(paired_speedup_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        paired_speedup_bounds(&[255, 1, 255, 2]),
        Some((253u32, 254u32))
    );
    assert_eq!(
        paired_speedup_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((0u32, 6u32))
    );
    assert_eq!(
        paired_speedup_bounds(&[17, 17, 1, 0, 2]),
        Some((0u32, 1u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2313::TRACK_PROOF, 0x52540000 | 2313);
}
