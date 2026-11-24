use task_2867::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_distinct(&[]), vec![]);
    assert_eq!(big_endian_registers_distinct(&[4]), vec![]);
}
