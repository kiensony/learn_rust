use task_1762::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_total(&[]), 0);
    assert_eq!(generated_offset_total(&[4]), 11);
}
