use task_2763::*;

#[test]
fn edge_cases() {
    assert_eq!(
        parsed_key_lengths_bounds("b=02;a=9;a=9;b=4"),
        Some((1u32, 1u32))
    );
    assert_eq!(
        parsed_key_lengths_bounds("missing;=4;c=256;a=x;;a=4"),
        Some((1u32, 1u32))
    );
    assert_eq!(
        parsed_key_lengths_bounds("a=0;b=1;b=2;z=255"),
        Some((1u32, 1u32))
    );
    assert_eq!(
        parsed_key_lengths_bounds("xx=3;z=0;LONG=12"),
        Some((1u32, 4u32))
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2763::TRACK_PROOF, 0x52540000 | 2763);
}
