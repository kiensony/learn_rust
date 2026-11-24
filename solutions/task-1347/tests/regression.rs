use task_1347::*;

#[test]
fn transfer_case() {
    assert_eq!(
        nibble_fields_run_ledger("4,9,2,2,0 4,9,2,2,0"),
        vec![(0u32, 8usize)]
    );
}
