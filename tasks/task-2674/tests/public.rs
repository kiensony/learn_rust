use task_2674::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_peaks(&[]), vec![]);
    assert_eq!(lending_checksum_peaks(&[4]), vec![4u32]);
}
