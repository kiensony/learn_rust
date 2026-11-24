use task_2663::*;

#[test]
fn public_examples() {
    assert_eq!(lending_first_byte_bounds(&[]), None);
    assert_eq!(lending_first_byte_bounds(&[4]), Some((4u32, 4u32)));
}
