use task_416::*;

#[test]
fn boundaries() {
    assert_eq!(next_cursor(0), 1);
    assert_eq!(next_cursor(255), 255);
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 416);
}
