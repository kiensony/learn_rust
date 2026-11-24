use task_2598::*;

#[test]
fn transfer_case() {
    assert_eq!(
        triplet_proof_rolling_load(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            3
        ),
        vec![18u64, 46u64, 60u64, 81u64, 56u64]
    );
}
