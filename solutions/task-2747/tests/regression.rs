use task_2747::*;

#[test]
fn transfer_case() {
    assert_eq!(
        cli_extensions_run_ledger(
            "input.release output.log /tmp/ok input.release output.log /tmp/ok"
        ),
        vec![
            (7u32, 1usize),
            (3u32, 1usize),
            (7u32, 1usize),
            (3u32, 1usize)
        ]
    );
}
