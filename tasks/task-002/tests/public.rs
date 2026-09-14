use task_002::classify;

#[test]
fn basics() {
    assert_eq!(classify(3), "fizz");
    assert_eq!(classify(5), "buzz");
    assert_eq!(classify(15), "fizzbuzz");
    assert_eq!(classify(1), "plain");
}
