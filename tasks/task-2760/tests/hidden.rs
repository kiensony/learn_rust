use task_2760::*;

#[test]
fn edge_cases() {
    assert_eq!(
        parsed_values_reverse("b=02;a=9;a=9;b=4"),
        vec![4u32, 9u32, 9u32, 2u32]
    );
    assert_eq!(
        parsed_values_reverse("missing;=4;c=256;a=x;;a=4"),
        vec![4u32]
    );
    assert_eq!(
        parsed_values_reverse("a=0;b=1;b=2;z=255"),
        vec![255u32, 2u32, 1u32, 0u32]
    );
    assert_eq!(
        parsed_values_reverse("xx=3;z=0;LONG=12"),
        vec![12u32, 0u32, 3u32]
    );
}

#[test]
fn submission_marker() {
    assert_eq!(task_2760::TRACK_PROOF, 0x52540000 | 2760);
}
