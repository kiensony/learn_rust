use task_2868::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_steps(&[]), vec![]);
    assert_eq!(big_endian_registers_steps(&[4]), vec![]);
}
