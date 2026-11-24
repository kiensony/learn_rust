mod common;

#[test]
fn examples() {
    common::check(&[], "Trip log\nUsage: trip <place>\n", "");
    common::check(&["unused"], "Trip log\nUsage: trip <place>\n", "");
}
