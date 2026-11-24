use task_1534::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_peaks(&[]), vec![]);
    assert_eq!(packet_gray_code_peaks(&[4]), vec![6u32]);
}
