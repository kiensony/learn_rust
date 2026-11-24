use task_2765::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_key_lengths_changes("b=02;a=9;a=9;b=4"), vec![1u32]);
    assert_eq!(
        parsed_key_lengths_changes("missing;=4;c=256;a=x;;a=4"),
        vec![1u32]
    );
    assert_eq!(parsed_key_lengths_changes("a=0;b=1;b=2;z=255"), vec![1u32]);
    assert_eq!(
        parsed_key_lengths_changes("xx=3;z=0;LONG=12"),
        vec![2u32, 1u32, 4u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2765::TRACK_PROOF, 0x52540000 | 2765);
}
