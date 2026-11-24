use task_905::*;

#[test]
fn example() {
    assert_eq!(quote(|a, b| a - b, 12, 4), 8);
}
