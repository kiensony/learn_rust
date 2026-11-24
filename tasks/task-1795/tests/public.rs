use task_1795::*;

#[test]
fn public_examples() {
    assert_eq!(token_offset_changes(&[]), vec![]);
    assert_eq!(token_offset_changes(&[4]), vec![9u32]);
}
