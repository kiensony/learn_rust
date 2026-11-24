use task_1318::*;

#[test]
fn edge_cases() {
    assert_eq!(
        nibble_fields_steps("4,9,2,2,0"),
        vec![0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(nibble_fields_steps(" , 12,xyz,-1,,255,256,"), vec![15u32]);
    assert_eq!(nibble_fields_steps("0,0,0"), vec![0u32, 0u32]);
    assert_eq!(
        nibble_fields_steps("12,3,12,4,5"),
        vec![0u32, 0u32, 0u32, 0u32]
    );
    assert_eq!(nibble_fields_steps("1, 02 ,3"), vec![0u32, 0u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1318::TRACK_PROOF, 0x52540000 | 1318);
}
