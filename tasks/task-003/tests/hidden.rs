use task_003::{parse_pair, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 3);
}

#[test]
fn trim_and_neg() {
    assert_eq!(parse_pair("  3 : -4  "), Ok((3, -4)));
}

#[test]
fn too_many() {
    assert_eq!(parse_pair("1:2:3"), Err("too many colons".into()));
}

#[test]
fn empty_side() {
    assert_eq!(parse_pair(":1"), Err("empty side".into()));
    assert_eq!(parse_pair("1:"), Err("empty side".into()));
    assert_eq!(parse_pair(" : "), Err("empty side".into()));
}

#[test]
fn not_i32() {
    assert_eq!(parse_pair("a:1"), Err("not an i32".into()));
    assert_eq!(parse_pair("1:9_001"), Err("not an i32".into()));
    assert_eq!(parse_pair("999999999999:1"), Err("not an i32".into()));
}
