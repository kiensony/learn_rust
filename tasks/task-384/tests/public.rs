use task_384::*;

#[test]
fn public_examples() {
    assert_eq!(mirror_count(&[1, 2, 1]), 1);
    assert_eq!(mirror_count(&[3, 4, 4, 3]), 2);
}
