use task_2998::*;

#[test]
fn public_examples() {
    assert_eq!(read_versioned(&[1, 1, 2]), Ok(513));
    assert_eq!(read_versioned(&[2, 1, 2]), Ok(258));
}
