use task_2965::*;

#[test]
fn edge_cases() {
    assert_eq!(loss_accounting_changes(&[9, 2, 2, 8, 0, 5]), vec![8u32]);
    assert_eq!(loss_accounting_changes(&[0, 0, 0]), vec![]);
    assert_eq!(loss_accounting_changes(&[255, 1, 255, 2]), vec![2u32]);
    assert_eq!(
        loss_accounting_changes(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31]),
        vec![12u32, 21u32, 15u32, 8u32, 31u32]
    );
    assert_eq!(loss_accounting_changes(&[17, 17, 1, 0, 2]), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2965::TRACK_PROOF, 0x52540000 | 2965);
}
