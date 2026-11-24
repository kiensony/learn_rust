use task_2774::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_peaks("b=02;a=9;a=9;b=4"), vec![9u32]);
    assert_eq!(repeated_keys_peaks("missing;=4;c=256;a=x;;a=4"), vec![]);
    assert_eq!(repeated_keys_peaks("a=0;b=1;b=2;z=255"), vec![2u32]);
    assert_eq!(repeated_keys_peaks("xx=3;z=0;LONG=12"), vec![]);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2774::TRACK_PROOF, 0x52540000 | 2774);
}
