use task_2678::*;

#[test]
fn public_examples() {
    assert_eq!(lending_checksum_steps(&[]), vec![]);
    assert_eq!(lending_checksum_steps(&[4]), vec![]);
}
