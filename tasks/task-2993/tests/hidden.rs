use task_2993::*;

#[test]
fn edge_cases() {
    assert_eq!(drain_order_bounds(&[9, 2, 2, 8, 0, 5]), Some((2u32, 8u32)));
    assert_eq!(drain_order_bounds(&[0, 0, 0]), None);
    assert_eq!(drain_order_bounds(&[255, 1, 255, 2]), Some((1u32, 255u32)));
    assert_eq!(
        drain_order_bounds(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        Some((6u32, 26u32))
    );
    assert_eq!(drain_order_bounds(&[17, 17, 1, 0, 2]), Some((1u32, 17u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_2993::TRACK_PROOF, 0x52540000 | 2993);
}
