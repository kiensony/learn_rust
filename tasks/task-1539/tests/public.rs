use task_1539::*;

#[test]
fn public_examples() {
    assert_eq!(packet_gray_code_longest_run(&[]), 0);
    assert_eq!(packet_gray_code_longest_run(&[4]), 1);
}
