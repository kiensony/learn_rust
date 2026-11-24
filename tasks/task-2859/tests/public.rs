use task_2859::*;

#[test]
fn public_examples() {
    assert_eq!(little_endian_registers_longest_run(&[]), 0);
    assert_eq!(little_endian_registers_longest_run(&[4]), 0);
}
