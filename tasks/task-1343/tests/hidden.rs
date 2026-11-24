use task_1343::*;

#[test]
fn edge_cases() {
    assert_eq!(packed_booleans_bounds("4,9,2,2,0"), Some((0u32, 2u32)));
    assert_eq!(
        packed_booleans_bounds(" , 12,xyz,-1,,255,256,"),
        Some((2u32, 8u32))
    );
    assert_eq!(packed_booleans_bounds("0,0,0"), Some((0u32, 0u32)));
    assert_eq!(packed_booleans_bounds("12,3,12,4,5"), Some((1u32, 2u32)));
    assert_eq!(packed_booleans_bounds("1, 02 ,3"), Some((1u32, 2u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1343::TRACK_PROOF, 0x52540000 | 1343);
}
