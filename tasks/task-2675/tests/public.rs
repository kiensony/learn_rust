use task_2675::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_changes(&[]), vec![]);
    assert_eq!(lending_checksum_changes(&[4]), vec![4u32]);
}
