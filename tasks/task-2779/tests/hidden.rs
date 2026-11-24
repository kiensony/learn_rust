use task_2779::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_longest_run("b=02;a=9;a=9;b=4"), 1);
    assert_eq!(repeated_keys_longest_run("missing;=4;c=256;a=x;;a=4"), 0);
    assert_eq!(repeated_keys_longest_run("a=0;b=1;b=2;z=255"), 1);
    assert_eq!(repeated_keys_longest_run("xx=3;z=0;LONG=12"), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2779::TRACK_PROOF, 0x52540000 | 2779);
}
