mod common;

#[test]
fn examples() {
    common::check(&["17"], "Item #17\nPainted clay\n", "");
    common::check(&["09"], "Item #09\nPainted clay\n", "");
}
