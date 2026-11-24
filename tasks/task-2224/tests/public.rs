use task_2224::*;

#[test]
fn public_examples() {
    assert_eq!(borrowed_scalars_peaks(""), vec![]);
    assert_eq!(borrowed_scalars_peaks("a"), vec![0u32]);
}
