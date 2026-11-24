use task_750::*;

#[test]
fn public_examples() {
    assert_eq!(first_ok(&[Err(()), Ok(8), Ok(9)]), Some(8));
    assert_eq!(first_ok(&[Ok(-2)]), Some(-2));
}
