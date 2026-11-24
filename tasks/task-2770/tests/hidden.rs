use task_2770::*;

#[test]
fn edge_cases() {
    assert_eq!(
        parsed_key_lengths_reverse("b=02;a=9;a=9;b=4"),
        vec![1u32, 1u32, 1u32, 1u32]
    );
    assert_eq!(
        parsed_key_lengths_reverse("missing;=4;c=256;a=x;;a=4"),
        vec![1u32]
    );
    assert_eq!(
        parsed_key_lengths_reverse("a=0;b=1;b=2;z=255"),
        vec![1u32, 1u32, 1u32, 1u32]
    );
    assert_eq!(
        parsed_key_lengths_reverse("xx=3;z=0;LONG=12"),
        vec![4u32, 1u32, 2u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2770::TRACK_PROOF, 0x52540000 | 2770);
}
