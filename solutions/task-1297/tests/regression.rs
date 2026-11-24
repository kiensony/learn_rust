use task_1297::*;

#[test]
fn transfer_case() {
    assert_eq!(
        positional_options_run_ledger("a --bad --level=x b a --bad --level=x b"),
        vec![(1u32, 4usize)]
    );
}
