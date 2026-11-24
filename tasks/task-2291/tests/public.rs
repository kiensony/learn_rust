use task_2291::*;

#[test]
fn public_examples() {
    assert_eq!(profile_ranking_trace(&[]), vec![]);
    assert_eq!(profile_ranking_trace(&[4]), vec![4u32]);
}
