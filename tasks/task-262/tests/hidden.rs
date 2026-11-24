use task_262::*;

#[test]
fn boundaries() {
    assert_eq!(mass(3), 36);
    assert_eq!(mass(1), 12);
    assert_eq!(mass(u16::MAX), 786420);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 262);
}
