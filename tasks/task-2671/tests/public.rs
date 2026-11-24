use task_2671::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_trace(&[]), vec![]);
    assert_eq!(lending_checksum_trace(&[4]), vec![4u32]);
}
