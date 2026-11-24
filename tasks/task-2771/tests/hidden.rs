use task_2771::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_trace("b=02;a=9;a=9;b=4"), vec![9u32, 4u32]);
    assert_eq!(repeated_keys_trace("missing;=4;c=256;a=x;;a=4"), vec![]);
    assert_eq!(repeated_keys_trace("a=0;b=1;b=2;z=255"), vec![2u32]);
    assert_eq!(repeated_keys_trace("xx=3;z=0;LONG=12"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2771::TRACK_PROOF, 0x52540000 | 2771);
}
