use task_2865::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_changes(&[]), vec![]);
    assert_eq!(big_endian_registers_changes(&[4]), vec![]);
}
