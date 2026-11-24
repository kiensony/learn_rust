use task_2193::*;

#[test]
fn edge_cases() {
    assert_eq!(
        duplicate_insertion_cost_bounds(&[9, 2, 2, 8, 0, 5]),
        Some((1u32, 5u32))
    );
    assert_eq!(
        duplicate_insertion_cost_bounds(&[0, 0, 0]),
        Some((1u32, 1u32))
    );
    assert_eq!(
        duplicate_insertion_cost_bounds(&[255, 1, 255, 2]),
        Some((1u32, 3u32))
    );
    assert_eq!(
        duplicate_insertion_cost_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((1u32, 10u32))
    );
    assert_eq!(
        duplicate_insertion_cost_bounds(&[17, 17, 1, 0, 2]),
        Some((1u32, 4u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2193::TRACK_PROOF, 0x52540000 | 2193);
}
