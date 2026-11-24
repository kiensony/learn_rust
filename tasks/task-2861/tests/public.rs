use task_2861::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_trace(&[]), vec![]);
    assert_eq!(big_endian_registers_trace(&[4]), vec![]);
}
