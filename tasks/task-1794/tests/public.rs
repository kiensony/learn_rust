use task_1794::*;

#[test]
fn public_examples() {
    assert_eq!(token_offset_peaks(&[]), vec![]);
    assert_eq!(token_offset_peaks(&[4]), vec![9u32]);
}
