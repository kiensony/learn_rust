use task_1977::*;

#[test]
fn public_examples() {
    assert_eq!(atomic_toggle_distinct(&[]), vec![]);
    assert_eq!(atomic_toggle_distinct(&[4]), vec![0u32]);
}
