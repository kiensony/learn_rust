use task_2294::*;

#[test]
fn public_examples() {
    assert_eq!(profile_ranking_peaks(&[]), vec![]);
    assert_eq!(profile_ranking_peaks(&[4]), vec![4u32]);
}
