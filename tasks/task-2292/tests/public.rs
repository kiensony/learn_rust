use task_2292::*;

#[test]
fn public_examples() {
    assert_eq!(profile_ranking_total(&[]), 0);
    assert_eq!(profile_ranking_total(&[4]), 4);
}
