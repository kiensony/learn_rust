use task_913::*;

#[test]
fn public_examples() {
    assert_eq!(visit_total(&[2, 3], |n| u32::from(n)), 5);
    assert_eq!(visit_total(&[2, 3], |n| u32::from(n) * 2), 10);
}
