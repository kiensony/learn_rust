use task_2996::*;

#[test]
fn public_examples() {
    assert_eq!(decode_batch(&[]), (Vec::<Vec<u8>>::new(), vec![]));
    assert_eq!(
        decode_batch(&[2, 4, 1, 0]),
        (vec![vec![4, 1], vec![]], vec![])
    );
}
