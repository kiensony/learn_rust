use task_1303::*;

#[test]
fn boundaries() {
    assert_eq!(port_width(None), 0);
    assert_eq!(port_width(Some(255)), 3);
    assert_eq!(port_width(Some(0)), 1);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 1303);
}
