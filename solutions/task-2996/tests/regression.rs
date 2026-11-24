use task_2996::*;

#[test]
fn transfer_case() {
    assert_eq!(decode_batch(&[255]), (vec![], vec![255]));
}

#[test]
fn resume_at_each_chunk_boundary() {
    let input = [2, 10, 11, 0, 1, 77];
    let (whole, _) = decode_batch(&input);
    for cut in 0..=input.len() {
        let (mut first, mut pending) = decode_batch(&input[..cut]);
        pending.extend_from_slice(&input[cut..]);
        let (second, tail) = decode_batch(&pending);
        first.extend(second);
        assert_eq!(first, whole);
        assert!(tail.is_empty());
    }
}
