use task_2667::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_distinct(&[]), vec![]);
    assert_eq!(lending_first_byte_distinct(&[4]), vec![4u32]);
}
