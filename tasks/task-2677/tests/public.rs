use task_2677::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_distinct(&[]), vec![]);
    assert_eq!(lending_checksum_distinct(&[4]), vec![4u32]);
}
