use task_2613::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_bounds(&[]), None);
    assert_eq!(embedded_flags_bounds(&[4]), Some((4u32, 4u32)));
}
