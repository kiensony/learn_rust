use task_101::*;

#[test]
fn public_examples() {
    assert_eq!(count_packs_10(65535, 4), 40);
    assert_eq!(count_packs_10(13, 14), 130);
}
