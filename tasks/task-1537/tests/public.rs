use task_1537::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_distinct(&[]), vec![]);
    assert_eq!(packet_gray_code_distinct(&[4]), vec![6u32]);
}
