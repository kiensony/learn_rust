use task_2798::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_rolling_load("a=1", 1), vec![]);
    assert_eq!(
        repeated_keys_rolling_load("b=02;a=9;a=9;b=4", 2),
        vec![13u64]
    );
    assert_eq!(
        repeated_keys_rolling_load("missing;=4;c=256;a=x;;a=4", 0),
        vec![]
    );
    assert_eq!(repeated_keys_rolling_load("a=0;b=1;b=2;z=255", 100), vec![]);
    assert_eq!(repeated_keys_rolling_load("xx=3;z=0;LONG=12", 3), vec![]);
    assert_eq!(
        repeated_keys_rolling_load("b=02;a=9;a=9;b=4", 2),
        vec![13u64]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2798::TRACK_PROOF, 0x52540000 | 2798);
}
