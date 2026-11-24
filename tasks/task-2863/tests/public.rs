use task_2863::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_bounds(&[]), None);
    assert_eq!(big_endian_registers_bounds(&[4]), None);
}
