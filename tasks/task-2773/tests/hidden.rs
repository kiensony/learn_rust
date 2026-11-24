use task_2773::*;

#[test]
fn edge_cases() {
    assert_eq!(repeated_keys_bounds("b=02;a=9;a=9;b=4"), Some((4u32, 9u32)));
    assert_eq!(repeated_keys_bounds("missing;=4;c=256;a=x;;a=4"), None);
    assert_eq!(
        repeated_keys_bounds("a=0;b=1;b=2;z=255"),
        Some((2u32, 2u32))
    );
    assert_eq!(repeated_keys_bounds("xx=3;z=0;LONG=12"), None);
}

#[test]
fn submission_marker() {
    assert_eq!(task_2773::TRACK_PROOF, 0x52540000 | 2773);
}
