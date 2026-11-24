use task_2661::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_trace(&[]), vec![]);
    assert_eq!(lending_first_byte_trace(&[4]), vec![4u32]);
}
