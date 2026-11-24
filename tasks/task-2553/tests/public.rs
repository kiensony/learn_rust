use task_2553::*;

#[test]
fn public_examples() {
    assert_eq!(split_proof_bounds(&[]), None);
    assert_eq!(split_proof_bounds(&[4]), Some((5u32, 5u32)));
}
