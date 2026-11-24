use task_2696::*;

#[test]
fn public_examples() {
    assert_eq!(
        lending_windows_threshold_crossing(&[3, 6, 9, 12, 21, 15, 0, 26, 8, 31], 0),
        Some((0, 2))
    );
    assert_eq!(lending_windows_threshold_crossing(&[], 0), None);
}
