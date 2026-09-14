use task_004::label;

#[test]
fn ada() {
    assert_eq!(label(String::from("ada"), 7), "7:ADA");
}
