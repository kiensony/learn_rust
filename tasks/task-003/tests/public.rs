use task_003::parse_pair;

#[test]
fn ok_simple() {
    assert_eq!(parse_pair("3:4"), Ok((3, 4)));
}

#[test]
fn missing() {
    assert_eq!(parse_pair("34"), Err("missing colon".into()));
}
