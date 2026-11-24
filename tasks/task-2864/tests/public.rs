use task_2864::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_peaks(&[]), vec![]);
    assert_eq!(big_endian_registers_peaks(&[4]), vec![]);
}
