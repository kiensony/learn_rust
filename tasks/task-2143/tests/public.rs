use task_2143::*;

#[test]
fn public_examples() {
    assert_eq!(first_occurrences_bounds(&[]), None);
    assert_eq!(first_occurrences_bounds(&[4]), Some((4u32, 4u32)));
}
