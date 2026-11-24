mod common;

#[test]
fn examples() {
    common::check(&["staff"], "[staff] staff\n", "");
    common::check(&["guest"], "[guest] guest\n", "");
}
