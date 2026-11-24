use task_119::*;

#[test]
fn boundaries() {
    assert_eq!(offset(55), 255);
    assert_eq!(offset(56), 255);
    assert_eq!(offset(0), 200);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 119);
}
