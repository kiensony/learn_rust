use task_2700::*;

#[test]
fn transfer_case() {
    assert_eq!(
        universal_slice_width_replay_mismatch(
            &[
                3, 6, 9, 12, 21, 15, 0, 26, 8, 31, 31, 8, 26, 0, 15, 21, 12, 9, 6, 3
            ],
            &[18, 31, 26, 3]
        ),
        None
    );
}
