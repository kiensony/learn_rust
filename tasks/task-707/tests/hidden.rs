use task_707::*;

#[test]
fn boundaries() {
    assert_eq!(reading(None), 0);
    assert_eq!(reading(Some(i16::MAX)), i16::MAX);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 707);
}
