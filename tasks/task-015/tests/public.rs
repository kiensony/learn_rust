mod common;

#[test]
fn examples() {
    common::check(&["Mina"], "Ticket: Mina\n", "");
    common::check(&["Pat"], "Ticket: Pat\n", "");
}
