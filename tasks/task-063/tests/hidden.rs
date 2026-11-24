use task_063::*;

#[test]
fn boundaries() {
    assert_eq!(degrees(0), 0);
    assert_eq!(degrees(800), 800);
    assert_eq!(degrees(i16::MIN), i16::MIN);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 63);
}
