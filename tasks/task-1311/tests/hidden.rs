use task_1311::*;

#[test]
fn edge_cases() {
    assert_eq!(
        nibble_fields_trace("4,9,2,2,0"),
        vec![0u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(
        nibble_fields_trace(" , 12,xyz,-1,,255,256,"),
        vec![0u32, 15u32]
    );
    assert_eq!(nibble_fields_trace("0,0,0"), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        nibble_fields_trace("12,3,12,4,5"),
        vec![0u32, 0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(nibble_fields_trace("1, 02 ,3"), vec![0u32, 0u32, 0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1311::TRACK_PROOF, 0x52540000 | 1311);
}
