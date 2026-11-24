use task_949::*;

#[test]
fn transfer_case() {
    assert_eq!(rises(&[-4, -3, -5, 0]), vec![-3, 0]);
}
