use task_1768::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_steps(&[]), vec![]);
    assert_eq!(generated_offset_steps(&[4]), vec![]);
}
