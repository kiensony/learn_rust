use task_2496::*;

#[test]
fn transfer_case() {
    assert_eq!(
        forward_raw_reads_threshold_crossing(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            1
        ),
        Some((0, 3))
    );
}
