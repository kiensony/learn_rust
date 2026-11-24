use task_2664::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_peaks(&[]), vec![]);
    assert_eq!(lending_first_byte_peaks(&[4]), vec![4u32]);
}
