use task_1430::*;

#[test]
fn edge_cases() {
    assert_eq!(
        remaining_budget_reverse(&[9, 2, 2, 8, 0, 5]),
        vec![35u32, 0u32, 86u32, 0u32, 0u32, 103u32]
    );
    assert_eq!(remaining_budget_reverse(&[0, 0, 0]), vec![0u32, 0u32, 0u32]);
    assert_eq!(
        remaining_budget_reverse(&[255, 1, 255, 2]),
        vec![0u32, 4285u32, 0u32, 4285u32]
    );
    assert_eq!(
        remaining_budget_reverse(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![
            477u32, 86u32, 392u32, 0u32, 205u32, 307u32, 154u32, 103u32, 52u32, 1u32
        ]
    );
    assert_eq!(
        remaining_budget_reverse(&[17, 17, 1, 0, 2]),
        vec![0u32, 0u32, 0u32, 239u32, 239u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1430::TRACK_PROOF, 0x52540000 | 1430);
}
