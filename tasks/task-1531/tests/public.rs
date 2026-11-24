use task_1531::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_trace(&[]), vec![]);
    assert_eq!(packet_gray_code_trace(&[4]), vec![6u32]);
}
