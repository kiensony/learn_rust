use task_2623::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_parity_bounds(&[]), None);
    assert_eq!(embedded_parity_bounds(&[4]), Some((1u32, 1u32)));
}
