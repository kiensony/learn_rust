use task_2673::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_bounds(&[]), None);
    assert_eq!(lending_checksum_bounds(&[4]), Some((4u32, 4u32)));
}
