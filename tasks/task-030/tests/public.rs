mod common;

#[test]
fn examples() {
    common::check(&["Kitchen"], "== Kitchen ==\n", "");
    common::check(&["Hall"], "== Hall ==\n", "");
}

#[test]
fn reusable_label() {
    assert_eq!(task_030::banner("Kitchen"), "== Kitchen ==");
    assert_eq!(task_030::banner("Hall"), "== Hall ==");
}
