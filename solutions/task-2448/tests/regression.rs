use task_2448::*;

#[test]
fn transfer_case() {
    assert_eq!(
        wide_alignment_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![
            32u64, 40u64, 56u64, 56u64, 40u64, 48u64, 40u64, 72u64, 72u64, 72u64, 72u64, 40u64,
            48u64, 40u64, 56u64, 56u64, 40u64, 32u64
        ]
    );
}
