use task_1304::*;

#[test]
fn edge_cases() {
    assert_eq!(decimal_fields_peaks("4,9,2,2,0"), vec![4u32, 9u32]);
    assert_eq!(
        decimal_fields_peaks(" , 12,xyz,-1,,255,256,"),
        vec![12u32, 255u32]
    );
    assert_eq!(decimal_fields_peaks("0,0,0"), vec![0u32]);
    assert_eq!(decimal_fields_peaks("12,3,12,4,5"), vec![12u32]);
    assert_eq!(decimal_fields_peaks("1, 02 ,3"), vec![1u32, 2u32, 3u32]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1304::TRACK_PROOF, 0x52540000 | 1304);
}
