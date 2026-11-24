use task_1347::*;

#[test]
fn edge_cases() {
    assert_eq!(nibble_fields_run_ledger("1"), vec![(0u32, 1usize)]);
    assert_eq!(nibble_fields_run_ledger("4,9,2,2,0"), vec![(0u32, 5usize)]);
    assert_eq!(
        nibble_fields_run_ledger(" , 12,xyz,-1,,255,256,"),
        vec![(0u32, 1usize), (15u32, 1usize)]
    );
    assert_eq!(nibble_fields_run_ledger("0,0,0"), vec![(0u32, 3usize)]);
    assert_eq!(
        nibble_fields_run_ledger("12,3,12,4,5"),
        vec![(0u32, 5usize)]
    );
    assert_eq!(nibble_fields_run_ledger("1, 02 ,3"), vec![(0u32, 3usize)]);
    assert_eq!(nibble_fields_run_ledger("4,9,2,2,0"), vec![(0u32, 5usize)]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_1347::TRACK_PROOF, 0x52540000 | 1347);
}
