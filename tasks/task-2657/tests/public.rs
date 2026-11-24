use task_2657::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_distinct(&[]), vec![]);
    assert_eq!(lending_windows_distinct(&[4]), vec![1u32]);
}
