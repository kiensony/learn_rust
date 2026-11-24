use task_2852::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_total(&[]), 0);
    assert_eq!(little_endian_registers_total(&[4]), 0);
}
