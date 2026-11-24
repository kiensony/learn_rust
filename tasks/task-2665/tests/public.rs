use task_2665::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_changes(&[]), vec![]);
    assert_eq!(lending_first_byte_changes(&[4]), vec![4u32]);
}
