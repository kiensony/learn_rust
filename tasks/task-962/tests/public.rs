use task_962::*;

#[test]
fn public_examples() {
    assert_eq!(legs(&[1, 4, 2]), vec![(1, 4), (4, 2)]);
    assert_eq!(legs(&[7, 7]), vec![(7, 7)]);
}
