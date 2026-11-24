use task_1010::*;

#[test]
fn boundaries() {
    assert_eq!(pair(0, 0), Ordered(0, 0));
    assert_eq!(pair(65535, 2), Ordered(65535, 2));
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1010);
}
