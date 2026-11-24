use task_2669::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_longest_run(&[]), 0);
    assert_eq!(lending_first_byte_longest_run(&[4]), 1);
}
