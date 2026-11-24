use task_1313::*;

#[test]
fn edge_cases() {
    assert_eq!(nibble_fields_bounds("4,9,2,2,0"), Some((0u32, 0u32)));
    assert_eq!(
        nibble_fields_bounds(" , 12,xyz,-1,,255,256,"),
        Some((0u32, 15u32))
    );
    assert_eq!(nibble_fields_bounds("0,0,0"), Some((0u32, 0u32)));
    assert_eq!(nibble_fields_bounds("12,3,12,4,5"), Some((0u32, 0u32)));
    assert_eq!(nibble_fields_bounds("1, 02 ,3"), Some((0u32, 0u32)));
}

#[test]
fn submission_marker() {
    assert_eq!(task_1313::TRACK_PROOF, 0x52540000 | 1313);
}
