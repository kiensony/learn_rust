use task_1540::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_reverse(&[]), vec![]);
    assert_eq!(packet_gray_code_reverse(&[4]), vec![6u32]);
}
