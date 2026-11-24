#[test]
fn all_three_worker_arrivals_agree() {
    let observations = [(3, 7), (3, 1), (5, 0)];
    for order in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let scheduled = order.map(|i| observations[i]);
        assert_eq!(
            task_1621::fastest(&scheduled),
            Some(1),
            "arrival order {order:?}"
        );
    }
}
