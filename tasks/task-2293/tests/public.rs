use task_2293::*;

#[test]
fn public_examples() {
    assert_eq!(profile_ranking_bounds(&[]), None);
    assert_eq!(profile_ranking_bounds(&[4]), Some((4u32, 4u32)));
}
