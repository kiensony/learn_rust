use task_228::*;

#[test]
fn public_examples() {
    assert_eq!(countdown_checksum_3(4, 0), 5);
    assert_eq!(countdown_checksum_3(13, 3), 51);
}
