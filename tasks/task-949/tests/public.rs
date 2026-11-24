use task_949::*;

#[test]
fn public_examples() {
    assert_eq!(rises(&[1, 3, 2, 4]), vec![3, 4]);
    assert_eq!(rises(&[4, 4, 3]), vec![]);
}
