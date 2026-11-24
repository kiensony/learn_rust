use task_1538::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_steps(&[]), vec![]);
    assert_eq!(packet_gray_code_steps(&[4]), vec![]);
}
