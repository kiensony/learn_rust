use task_1004::*;

#[test]
fn boundaries() {
    assert_eq!(clear::<0>(), [0; 0]);
    assert_eq!(clear::<7>(), [0; 7]);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1004);
}
