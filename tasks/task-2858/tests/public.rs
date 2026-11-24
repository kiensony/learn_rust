use task_2858::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_steps(&[]), vec![]);
    assert_eq!(little_endian_registers_steps(&[4]), vec![]);
}
