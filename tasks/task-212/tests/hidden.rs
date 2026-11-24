use task_212::*;

#[test]
fn boundaries() {
    assert_eq!(first_even(&[]), None);
    assert_eq!(first_even(&[1, 3]), None);
    assert_eq!(first_even(&[0]), Some(0));
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 212);
}
