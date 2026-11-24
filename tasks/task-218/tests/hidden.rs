use task_218::*;

#[test]
fn boundaries() {
    assert_eq!(stop(0), 0);
    assert_eq!(stop(-6), 0);
    assert_eq!(stop(i16::MAX), 32767);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 218);
}
