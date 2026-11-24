mod common;

#[test]
fn examples() {
    common::check(&[], "---\nNew timetable\n---\n", "");
    common::check(&["unused"], "---\nNew timetable\n---\n", "");
}
