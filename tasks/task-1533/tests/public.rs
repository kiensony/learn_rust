use task_1533::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_bounds(&[]), None);
    assert_eq!(packet_gray_code_bounds(&[4]), Some((6u32, 6u32)));
}
