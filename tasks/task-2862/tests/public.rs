use task_2862::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_total(&[]), 0);
    assert_eq!(big_endian_registers_total(&[4]), 0);
}
