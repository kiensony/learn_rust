use task_2757::*;

#[test]
fn edge_cases() {
    assert_eq!(
        parsed_values_distinct("b=02;a=9;a=9;b=4"),
        vec![2u32, 4u32, 9u32]
    );
    assert_eq!(
        parsed_values_distinct("missing;=4;c=256;a=x;;a=4"),
        vec![4u32]
    );
    assert_eq!(
        parsed_values_distinct("a=0;b=1;b=2;z=255"),
        vec![0u32, 1u32, 2u32, 255u32]
    );
    assert_eq!(
        parsed_values_distinct("xx=3;z=0;LONG=12"),
        vec![0u32, 3u32, 12u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2757::TRACK_PROOF, 0x52540000 | 2757);
}
