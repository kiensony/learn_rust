use task_2855::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_changes(&[]), vec![]);
    assert_eq!(little_endian_registers_changes(&[4]), vec![]);
}
