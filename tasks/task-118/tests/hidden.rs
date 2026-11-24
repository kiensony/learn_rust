use task_118::*;

#[test]
fn edge_cases() {
    assert_eq!(digit_place_2(13), 0);
    assert_eq!(digit_place_2(0), 0);
    assert_eq!(digit_place_2(1), 0);
    assert_eq!(digit_place_2(1), 0);
    assert_eq!(digit_place_2(3), 1);
    assert_eq!(digit_place_2(65535), 1);
    assert_eq!(digit_place_2(20), 0);
    assert_eq!(digit_place_2(40), 0);
    assert_eq!(digit_place_2(50), 1);
    assert_eq!(digit_place_2(100), 0);
    assert_eq!(digit_place_2(101), 0);
    assert_eq!(digit_place_2(255), 1);
    assert_eq!(digit_place_2(256), 0);
}

#[test]
fn submission_marker() {
    assert_eq!(task_118::TRACK_PROOF, 0x52540000 | 118);
}
