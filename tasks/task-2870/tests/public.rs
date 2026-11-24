use task_2870::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_reverse(&[]), vec![]);
    assert_eq!(big_endian_registers_reverse(&[4]), vec![]);
}
