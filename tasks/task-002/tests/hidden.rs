use task_002::{classify, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 2);
}

#[test]
fn zero_and_negatives() {
    assert_eq!(classify(0), "fizzbuzz");
    assert_eq!(classify(-3), "fizz");
    assert_eq!(classify(-5), "buzz");
    assert_eq!(classify(-15), "fizzbuzz");
    assert_eq!(classify(-1), "plain");
}

#[test]
fn large() {
    assert_eq!(classify(10_000_000_000_000_000), "buzz");
    assert_eq!(classify(9_000_000_000_000_003), "fizz");
}
