use task_2753::*;

#[test]
fn edge_cases() {
    assert_eq!(parsed_values_bounds("b=02;a=9;a=9;b=4"), Some((2u32, 9u32)));
    assert_eq!(
        parsed_values_bounds("missing;=4;c=256;a=x;;a=4"),
        Some((4u32, 4u32))
    );
    assert_eq!(
        parsed_values_bounds("a=0;b=1;b=2;z=255"),
        Some((0u32, 255u32))
    );
    assert_eq!(
        parsed_values_bounds("xx=3;z=0;LONG=12"),
        Some((0u32, 12u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2753::TRACK_PROOF, 0x52540000 | 2753);
}
