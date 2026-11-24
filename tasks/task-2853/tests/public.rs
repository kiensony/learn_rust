use task_2853::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_bounds(&[]), None);
    assert_eq!(little_endian_registers_bounds(&[4]), None);
}
