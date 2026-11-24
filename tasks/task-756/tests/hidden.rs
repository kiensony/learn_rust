use task_756::*;

#[test]
fn boundaries() {
    assert_eq!(stops(&[]), vec![0]);
    assert_eq!(stops(&[0, 0]), vec![0, 0, 0]);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 756);
}
