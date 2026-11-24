use task_1347::*;

#[test]
fn public_examples() {
    assert_eq!(nibble_fields_run_ledger("4,9,2,2,0"), vec![(0u32, 5usize)]);
    assert_eq!(nibble_fields_run_ledger(""), vec![]);
}
