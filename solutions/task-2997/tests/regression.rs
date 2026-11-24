use task_2997::*;

#[test]
fn transfer_case() {
    assert_eq!(reconcile(&[(12, 0), (12, 3)]), Err(12));
}

#[test]
fn report_the_first_conflict_in_arrival_order() {
    let first = [(8, 2), (3, 5), (8, 1), (3, 0)];
    let second = [(8, 2), (3, 5), (3, 0), (8, 1)];
    assert_eq!(reconcile(&first), Err(8));
    assert_eq!(reconcile(&second), Err(3));
}
