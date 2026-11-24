use task_609::*;

#[test]
fn boundaries() {
    assert_eq!(locate(0, -2), Point { x: 0, y: -2 });
    assert_eq!(locate(-5, 0), Point { x: -5, y: 0 });
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 609);
}
