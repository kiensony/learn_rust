use task_2670::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_reverse(&[]), vec![]);
    assert_eq!(lending_first_byte_reverse(&[4]), vec![4u32]);
}
