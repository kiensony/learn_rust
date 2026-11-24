use task_1791::*;

#[test]
fn public_examples() {
    assert_eq!(token_offset_trace(&[]), vec![]);
    assert_eq!(token_offset_trace(&[4]), vec![9u32]);
}
