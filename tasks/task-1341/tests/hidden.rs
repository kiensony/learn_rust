use task_1341::*;

#[test]
fn edge_cases() {
    assert_eq!(
        packed_booleans_trace("4,9,2,2,0"),
        vec![1u32, 2u32, 1u32, 1u32, 0u32]
    );
    assert_eq!(
        packed_booleans_trace(" , 12,xyz,-1,,255,256,"),
        vec![2u32, 8u32]
    );
    assert_eq!(packed_booleans_trace("0,0,0"), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        packed_booleans_trace("12,3,12,4,5"),
        vec![2u32, 2u32, 2u32, 1u32, 2u32]
    );
    assert_eq!(packed_booleans_trace("1, 02 ,3"), vec![1u32, 1u32, 2u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1341::TRACK_PROOF, 0x52540000 | 1341);
}
