use task_2344::*;

#[test]
fn public_examples() {
    assert_eq!(checksum(&[]), 0);
    assert_eq!(checksum(&[1]), 1);
}
