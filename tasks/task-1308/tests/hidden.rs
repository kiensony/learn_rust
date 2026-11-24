use task_1308::*;

#[test]
fn edge_cases() {
    assert_eq!(
        decimal_fields_steps("4,9,2,2,0"),
        vec![5u32, 7u32, 0u32, 2u32]
    );
    assert_eq!(decimal_fields_steps(" , 12,xyz,-1,,255,256,"), vec![243u32]);
    assert_eq!(decimal_fields_steps("0,0,0"), vec![0u32, 0u32]);
    assert_eq!(
        decimal_fields_steps("12,3,12,4,5"),
        vec![9u32, 9u32, 8u32, 1u32]
    );
    assert_eq!(decimal_fields_steps("1, 02 ,3"), vec![1u32, 1u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1308::TRACK_PROOF, 0x52540000 | 1308);
}
