use task_2653::*;

#[test]
fn public_examples() {
    assert_eq!(lending_windows_bounds(&[]), None);
    assert_eq!(lending_windows_bounds(&[4]), Some((1u32, 1u32)));
}
