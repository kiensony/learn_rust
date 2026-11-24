mod common;

#[test]
fn examples() {
    common::check(&[], "Press \"Enter\" to continue.\n", "");
    common::check(&["unused"], "Press \"Enter\" to continue.\n", "");
}
