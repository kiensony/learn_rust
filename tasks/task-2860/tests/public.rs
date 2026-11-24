use task_2860::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_reverse(&[]), vec![]);
    assert_eq!(little_endian_registers_reverse(&[4]), vec![]);
}
