use task_2854::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_peaks(&[]), vec![]);
    assert_eq!(little_endian_registers_peaks(&[4]), vec![]);
}
