use task_1127::*;

#[test]
fn public_examples() {
    assert_eq!(byte_label(vec![2, 3]), "2 bytes");
    assert_eq!(byte_label(vec![8]), "1 bytes");
}
