use task_2866::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_descending_steps(&[]), 0);
    assert_eq!(big_endian_registers_descending_steps(&[4]), 0);
}
