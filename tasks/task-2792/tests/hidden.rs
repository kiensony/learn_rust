use task_2792::*;

#[test]
fn edge_cases() {
    assert_eq!(final_settings_total("b=02;a=9;a=9;b=4"), 13);
    assert_eq!(final_settings_total("missing;=4;c=256;a=x;;a=4"), 4);
    assert_eq!(final_settings_total("a=0;b=1;b=2;z=255"), 257);
    assert_eq!(final_settings_total("xx=3;z=0;LONG=12"), 15);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2792::TRACK_PROOF, 0x52540000 | 2792);
}
