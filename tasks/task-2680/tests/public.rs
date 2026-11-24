use task_2680::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_reverse(&[]), vec![]);
    assert_eq!(lending_checksum_reverse(&[4]), vec![4u32]);
}
