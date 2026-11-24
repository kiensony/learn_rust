use task_2857::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_distinct(&[]), vec![]);
    assert_eq!(little_endian_registers_distinct(&[4]), vec![]);
}
