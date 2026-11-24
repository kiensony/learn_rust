use task_504::*;

#[test]
fn transfer_case() {
    assert_eq!(
        columns(&[(-128, 127), (127, -128)]),
        (vec![-128, 127], vec![127, -128])
    );
}
