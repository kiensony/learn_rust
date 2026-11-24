use task_2851::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_trace(&[]), vec![]);
    assert_eq!(little_endian_registers_trace(&[4]), vec![]);
}
