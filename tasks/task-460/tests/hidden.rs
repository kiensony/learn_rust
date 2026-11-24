use task_460::*;

#[test]
fn boundaries() {
    assert_eq!(height((5, -3)), -3);
    assert_eq!(height((7, 0)), 0);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 460);
}
