use task_2797::*;

#[test]
fn transfer_case() {
    assert_eq!(
        parsed_key_lengths_run_ledger("xx=3;z=0;LONG=12 xx=3;z=0;LONG=12"),
        vec![(2u32, 1usize), (1u32, 2usize), (4u32, 1usize)]
    );
}
