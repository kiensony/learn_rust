mod common;

#[test]
fn examples() {
    common::check(&["Pier", "Monday"], "Pier / Monday\n", "");
    common::check(&["Hill", "Friday"], "Hill / Friday\n", "");
}
