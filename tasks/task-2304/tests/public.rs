use task_2304::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_peaks(&[]), vec![]);
    assert_eq!(warmup_discard_peaks(&[4]), vec![]);
}
