use task_1980::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_reverse(&[]), vec![]);
    assert_eq!(atomic_toggle_reverse(&[4]), vec![0u32]);
}
