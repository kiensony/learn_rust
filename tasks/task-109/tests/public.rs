use task_109::*;

#[test]
fn public_examples() {
    assert_eq!(a_full_tray_10(65535, 4), false);
    assert_eq!(a_full_tray_10(65535, 0), true);
}
