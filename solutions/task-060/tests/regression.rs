mod common;

#[test]
fn keep_roles_distinct() { common::check(&["go", "A"], "A <- go\n", ""); }
