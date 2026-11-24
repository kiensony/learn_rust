use task_2303::*;

#[test]
fn public_examples() {
    assert_eq!(warmup_discard_bounds(&[]), None);
    assert_eq!(warmup_discard_bounds(&[4]), None);
}
