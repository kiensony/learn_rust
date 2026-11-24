use task_1535::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_changes(&[]), vec![]);
    assert_eq!(packet_gray_code_changes(&[4]), vec![6u32]);
}
