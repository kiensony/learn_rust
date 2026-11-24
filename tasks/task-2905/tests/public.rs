use task_2905::*;

#[test]
fn public_examples() {
    assert_eq!(latency_increases_changes(&[]), vec![]);
    assert_eq!(latency_increases_changes(&[4]), vec![]);
}
