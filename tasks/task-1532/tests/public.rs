use task_1532::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_total(&[]), 0);
    assert_eq!(packet_gray_code_total(&[4]), 6);
}
