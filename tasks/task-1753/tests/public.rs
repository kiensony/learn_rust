use task_1753::*;

#[test]
fn public_examples() {
    assert_eq!(build_multiplier_bounds(&[]), None);
    assert_eq!(build_multiplier_bounds(&[4]), Some((12u32, 12u32)));
}
