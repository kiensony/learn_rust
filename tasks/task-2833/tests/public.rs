use task_2833::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_bounds(&[]), None);
    assert_eq!(service_expansion_bounds(&[4]), Some((8u32, 8u32)));
}
