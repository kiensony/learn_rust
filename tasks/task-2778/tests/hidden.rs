use task_2778::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_steps("b=02;a=9;a=9;b=4"), vec![5u32]);
    assert_eq!(repeated_keys_steps("missing;=4;c=256;a=x;;a=4"), vec![]);
    assert_eq!(repeated_keys_steps("a=0;b=1;b=2;z=255"), vec![]);
    assert_eq!(repeated_keys_steps("xx=3;z=0;LONG=12"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2778::TRACK_PROOF, 0x52540000 | 2778);
}
