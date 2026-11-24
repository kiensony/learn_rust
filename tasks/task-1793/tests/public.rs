use task_1793::*;

#[test]
fn public_examples() {
    assert_eq!(token_offset_bounds(&[]), None);
    assert_eq!(token_offset_bounds(&[4]), Some((9u32, 9u32)));
}
