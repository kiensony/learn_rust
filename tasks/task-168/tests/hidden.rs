use task_168::*;

#[test]
fn boundaries() {
    assert_eq!(status(1), "online");
    assert_eq!(status(u32::MAX), "online");
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 168);
}
