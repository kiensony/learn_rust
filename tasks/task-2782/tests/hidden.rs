use task_2782::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_defaults_total("b=02;a=9;a=9;b=4"), 0);
    assert_eq!(parsed_defaults_total("missing;=4;c=256;a=x;;a=4"), 0);
    assert_eq!(parsed_defaults_total("a=0;b=1;b=2;z=255"), 1);
    assert_eq!(parsed_defaults_total("xx=3;z=0;LONG=12"), 1);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2782::TRACK_PROOF, 0x52540000 | 2782);
}
