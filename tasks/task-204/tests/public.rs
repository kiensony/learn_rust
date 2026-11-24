use task_204::*;

#[test]
fn public_examples() {
    assert_eq!(even_batch_10(65535, 4), false);
    assert_eq!(even_batch_10(10, 11), true);
}
