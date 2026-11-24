use task_2772::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_total("b=02;a=9;a=9;b=4"), 13);
    assert_eq!(repeated_keys_total("missing;=4;c=256;a=x;;a=4"), 0);
    assert_eq!(repeated_keys_total("a=0;b=1;b=2;z=255"), 2);
    assert_eq!(repeated_keys_total("xx=3;z=0;LONG=12"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2772::TRACK_PROOF, 0x52540000 | 2772);
}
