use task_2752::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_values_total("b=02;a=9;a=9;b=4"), 24);
    assert_eq!(parsed_values_total("missing;=4;c=256;a=x;;a=4"), 4);
    assert_eq!(parsed_values_total("a=0;b=1;b=2;z=255"), 258);
    assert_eq!(parsed_values_total("xx=3;z=0;LONG=12"), 15);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2752::TRACK_PROOF, 0x52540000 | 2752);
}
