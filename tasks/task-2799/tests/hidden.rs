use task_2799::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_defaults_frequent_readings("a=1", 0), vec![]);
    assert_eq!(
        parsed_defaults_frequent_readings("b=02;a=9;a=9;b=4", 1),
        vec![]
    );
    assert_eq!(
        parsed_defaults_frequent_readings("missing;=4;c=256;a=x;;a=4", 2),
        vec![]
    );
    assert_eq!(
        parsed_defaults_frequent_readings("a=0;b=1;b=2;z=255", 3),
        vec![1u32]
    );
    assert_eq!(
        parsed_defaults_frequent_readings("xx=3;z=0;LONG=12", 256),
        vec![1u32]
    );
    assert_eq!(
        parsed_defaults_frequent_readings("a=0;b=1;b=2;z=255", 1),
        vec![1u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2799::TRACK_PROOF, 0x52540000 | 2799);
}
