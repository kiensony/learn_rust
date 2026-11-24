use task_1234::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_peaks(&[]), vec![]);
    assert_eq!(seek_offsets_peaks(&[4]), vec![]);
}
