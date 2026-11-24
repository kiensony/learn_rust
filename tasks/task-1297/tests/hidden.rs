use task_1297::*;

#[test]
fn edge_cases() {
    assert_eq!(positional_options_run_ledger("only"), vec![(4u32, 1usize)]);
    assert_eq!(positional_options_run_ledger("-q -q -- -v"), vec![]);
    assert_eq!(
        positional_options_run_ledger("--level=3 data --level=9"),
        vec![(4u32, 1usize)]
    );
    assert_eq!(
        positional_options_run_ledger("a --bad --level=x b"),
        vec![(1u32, 2usize)]
    );
    assert_eq!(
        positional_options_run_ledger("input -v -q input --level=255"),
        vec![(5u32, 2usize)]
    );
    assert_eq!(
        positional_options_run_ledger("-- --level=4 x"),
        vec![(1u32, 1usize)]
    );
    assert_eq!(
        positional_options_run_ledger("a --bad --level=x b"),
        vec![(1u32, 2usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_1297::TRACK_PROOF, 0x52540000 | 1297);
}
