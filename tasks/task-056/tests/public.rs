mod common;

#[test]
fn examples() {
    common::check(&["Ridge", "Tue"], "TRAIL CARD\n[Ridge]\n(Tue)\n", "");
    common::check(&["West Gate", "Mon"], "TRAIL CARD\n[West Gate]\n(Mon)\n", "");
}
