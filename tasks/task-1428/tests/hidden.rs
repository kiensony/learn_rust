use task_1428::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remaining_budget_steps(&[9, 2, 2, 8, 0, 5]),
        vec![103u32, 0u32, 86u32, 86u32, 35u32]
    );
    assert_eq!(remaining_budget_steps(&[0, 0, 0]), vec![0u32, 0u32]);
    assert_eq!(
        remaining_budget_steps(&[255, 1, 255, 2]),
        vec![4285u32, 4285u32, 4285u32]
    );
    assert_eq!(
        remaining_budget_steps(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            51u32, 51u32, 51u32, 153u32, 102u32, 205u32, 392u32, 306u32, 391u32
        ]
    );
    assert_eq!(
        remaining_budget_steps(&[17, 17, 1, 0, 2]),
        vec![0u32, 239u32, 0u32, 0u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1428::TRACK_PROOF, 0x52540000 | 1428);
}
