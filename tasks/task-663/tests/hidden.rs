use task_663::*;

#[test]
fn boundaries() {
    assert_eq!(weight(Shipment::Letter(0)), 0);
    assert_eq!(weight(Shipment::Box(255, 255)), 510);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 663);
}
