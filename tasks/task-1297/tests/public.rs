use task_1297::*;

#[test]
fn public_examples() {
    assert_eq!(
        positional_options_run_ledger("a --bad --level=x b"),
        vec![(1u32, 2usize)]
    );
    assert_eq!(positional_options_run_ledger(""), vec![]);
}
