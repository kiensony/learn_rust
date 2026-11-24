use task_2768::*;

#[test]
fn edge_cases() {
    assert_eq!(
        parsed_key_lengths_steps("b=02;a=9;a=9;b=4"),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        parsed_key_lengths_steps("missing;=4;c=256;a=x;;a=4"),
        vec![]
    );
    assert_eq!(
        parsed_key_lengths_steps("a=0;b=1;b=2;z=255"),
        vec![0u32, 0u32, 0u32]
    );
    assert_eq!(
        parsed_key_lengths_steps("xx=3;z=0;LONG=12"),
        vec![1u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2768::TRACK_PROOF, 0x52540000 | 2768);
}
