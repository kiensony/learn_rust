use task_2603::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_checksum_bounds(&[]), None);
    assert_eq!(embedded_checksum_bounds(&[4]), Some((2u32, 2u32)));
}
