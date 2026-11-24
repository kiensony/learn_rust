use task_2393::*;

#[test]
fn public_examples() {
    assert_eq!(last_change_age_bounds(&[]), None);
    assert_eq!(last_change_age_bounds(&[4]), Some((1u32, 1u32)));
}
