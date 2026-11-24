use task_1048::*;

#[test]
fn transfer_case() {
    assert_eq!(
        zip_array([String::from("x")], [false]),
        [(String::from("x"), false)]
    );
}
