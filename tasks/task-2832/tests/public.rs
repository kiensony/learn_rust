use task_2832::*;

#[test]
fn public_examples() {
    assert_eq!(service_expansion_total(&[]), 0);
    assert_eq!(service_expansion_total(&[4]), 8);
}
