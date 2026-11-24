use task_2796::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_values_threshold_crossing("a=1", 0), Some((0, 1)));
    assert_eq!(
        parsed_values_threshold_crossing("b=02;a=9;a=9;b=4", 1),
        Some((0, 2))
    );
    assert_eq!(
        parsed_values_threshold_crossing("missing;=4;c=256;a=x;;a=4", 4),
        Some((0, 4))
    );
    assert_eq!(
        parsed_values_threshold_crossing("a=0;b=1;b=2;z=255", 256),
        None
    );
    assert_eq!(
        parsed_values_threshold_crossing("xx=3;z=0;LONG=12", 0),
        Some((0, 3))
    );
    assert_eq!(
        parsed_values_threshold_crossing("a=0;b=1;b=2;z=255", 256),
        None
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2796::TRACK_PROOF, 0x52540000 | 2796);
}
