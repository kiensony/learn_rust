mod common;

#[test]
fn examples() {
    common::check(&[], "\n", "");
    common::check(&["Hugo"], "Hugo\n", "");
}
