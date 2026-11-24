use task_2869::*;

#[test]
fn public_examples() {
    assert_eq!(big_endian_registers_longest_run(&[]), 0);
    assert_eq!(big_endian_registers_longest_run(&[4]), 0);
}
