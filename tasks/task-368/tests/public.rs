use task_368::*;

#[test]
fn public_examples() {
    assert_eq!(label_info(&String::from("Lake")), (4, true));
    assert_eq!(label_info(&String::from("trail map")), (9, true));
}
