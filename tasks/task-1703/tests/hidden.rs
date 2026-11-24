use task_1703::*;

#[test]
fn boundaries() {
    assert_eq!(Meter(0).value(), 0);
    assert_eq!(Meter(65535).value(), 65535);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1703);
}
