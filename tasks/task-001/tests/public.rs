use task_001::greet;

#[test]
fn hello_ada() {
    assert_eq!(greet("Ada"), "Hello, Ada!");
}

#[test]
fn stranger_empty() {
    assert_eq!(greet(""), "Hello, stranger!");
}
