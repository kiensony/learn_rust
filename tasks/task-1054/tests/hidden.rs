use task_1054::*;

#[test]
fn boundaries() {
    assert_eq!(Shelf(0).slots(), 0);
    assert_eq!(Shelf(u16::MAX).slots(), u16::MAX);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1054);
}
