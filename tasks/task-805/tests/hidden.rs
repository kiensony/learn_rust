use task_805::*;

#[test]
fn boundaries() {
    assert_eq!(parse_count(""), Err(BadCount));
    assert_eq!(parse_count("-4"), Err(BadCount));
    assert_eq!(parse_count("65536"), Err(BadCount));
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52540000 | 805);
}
