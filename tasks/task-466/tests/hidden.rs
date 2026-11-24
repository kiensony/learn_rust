use task_466::*;

#[test]
fn boundaries() {
    assert_eq!(red([0, 255, 2]), 0);
    assert_eq!(red([255, 0, 1]), 255);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 466);
}
