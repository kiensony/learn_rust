use task_2662::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_total(&[]), 0);
    assert_eq!(lending_first_byte_total(&[4]), 4);
}
