mod common;

#[test]
fn examples() {
    common::check(&["rain"], "===\nrain\n===\n", "");
    common::check(&["wind"], "===\nwind\n===\n", "");
}

#[test]
fn reusable_value() {
    assert_eq!(task_047::divider(), "===");
}
