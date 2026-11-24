use task_106::*;

#[test]
fn public_examples() {
    assert_eq!(bundle_labels_10(65535, 4), 44);
    assert_eq!(bundle_labels_10(13, 14), 143);
}
