use task_2997::*;

#[test]
fn public_examples() {
    assert_eq!(reconcile(&[]), Ok(0));
    assert_eq!(reconcile(&[(1, 7), (2, 5), (1, 7)]), Ok(12));
}
