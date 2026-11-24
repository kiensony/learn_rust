use task_2683::*;

#[test]
fn edge_cases() {
    assert_eq!(
        universal_slice_sum_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((5u32, 21u32))
    );
    assert_eq!(universal_slice_sum_bounds(&[0, 0, 0]), Some((0u32, 0u32)));
    assert_eq!(
        universal_slice_sum_bounds(&[255, 1, 255, 2]),
        Some((513u32, 513u32))
    );
    assert_eq!(
        universal_slice_sum_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((51u32, 80u32))
    );
    assert_eq!(
        universal_slice_sum_bounds(&[17, 17, 1, 0, 2]),
        Some((37u32, 37u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2683::TRACK_PROOF, 0x52540000 | 2683);
}
