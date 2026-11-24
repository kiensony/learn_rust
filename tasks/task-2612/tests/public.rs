use task_2612::*;

#[test]
fn public_examples() {
    assert_eq!(embedded_flags_total(&[]), 0);
    assert_eq!(embedded_flags_total(&[4]), 4);
}
