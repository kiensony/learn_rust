use task_163::*;

#[test]
fn boundaries() {
    assert_eq!(price(false), 2);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 163);
}
