use task_2295::*;

#[test]
fn public_examples() {
    assert_eq!(profile_ranking_changes(&[]), vec![]);
    assert_eq!(profile_ranking_changes(&[4]), vec![4u32]);
}
