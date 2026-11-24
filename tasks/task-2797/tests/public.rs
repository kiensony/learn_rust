use task_2797::*;

#[test]
fn public_examples() {
    assert_eq!(
        parsed_key_lengths_run_ledger("xx=3;z=0;LONG=12"),
        vec![(2u32, 1usize), (1u32, 1usize), (4u32, 1usize)]
    );
    assert_eq!(parsed_key_lengths_run_ledger(""), vec![]);
}
