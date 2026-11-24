use task_1763::*;

#[test]
fn public_examples() {
    assert_eq!(generated_offset_bounds(&[]), None);
    assert_eq!(generated_offset_bounds(&[4]), Some((11u32, 11u32)));
}
