use task_2599::*;

#[test]
fn public_examples() {
    assert_eq!(
        stride_proof_frequent_readings(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 3),
        vec![7u32, 22u32, 27u32]
    );
    assert_eq!(stride_proof_frequent_readings(&[], 0), vec![]);
}
