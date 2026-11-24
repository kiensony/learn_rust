use task_1453::*;

#[test]
fn public_examples() {
    assert_eq!(http_status_fixture_bounds(""), None);
    assert_eq!(http_status_fixture_bounds("200"), Some((2u32, 2u32)));
}
