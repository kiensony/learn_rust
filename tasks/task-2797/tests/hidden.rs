use task_2797::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_key_lengths_run_ledger("a=1"), vec![(1u32, 1usize)]);
    assert_eq!(
        parsed_key_lengths_run_ledger("b=02;a=9;a=9;b=4"),
        vec![(1u32, 4usize)]
    );
    assert_eq!(
        parsed_key_lengths_run_ledger("missing;=4;c=256;a=x;;a=4"),
        vec![(1u32, 1usize)]
    );
    assert_eq!(
        parsed_key_lengths_run_ledger("a=0;b=1;b=2;z=255"),
        vec![(1u32, 4usize)]
    );
    assert_eq!(
        parsed_key_lengths_run_ledger("xx=3;z=0;LONG=12"),
        vec![(2u32, 1usize), (1u32, 1usize), (4u32, 1usize)]
    );
    assert_eq!(
        parsed_key_lengths_run_ledger("xx=3;z=0;LONG=12"),
        vec![(2u32, 1usize), (1u32, 1usize), (4u32, 1usize)]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2797::TRACK_PROOF, 0x52540000 | 2797);
}
