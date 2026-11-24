use task_1103::*;

#[test]
fn boundaries() {
    assert_eq!(cost(&0u8), 0);
    assert_eq!(cost(&255u8), 255);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1103);
}
