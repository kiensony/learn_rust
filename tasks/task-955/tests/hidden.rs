use task_955::*;

#[test]
fn boundaries() {
    assert_eq!(twice_all(&[]), 0);
    assert_eq!(twice_all(&[255, 255]), 1020);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 955);
}
