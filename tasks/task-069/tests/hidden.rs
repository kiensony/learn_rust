use task_069::*;

#[test]
fn boundaries() {
    assert_eq!(busy(false, false), false);
    assert_eq!(busy(true, false), true);
    assert_eq!(busy(false, true), false);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 69);
}
