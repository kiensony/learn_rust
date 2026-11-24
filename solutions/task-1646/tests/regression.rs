use task_1646::*;

#[test]
fn transfer_case() {
    assert_eq!(
        parity_property_threshold_crossing(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            1
        ),
        None
    );
}
