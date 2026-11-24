use task_1792::*;

#[test]
fn public_examples() {
    assert_eq!(token_offset_total(&[]), 0);
    assert_eq!(token_offset_total(&[4]), 9);
}
