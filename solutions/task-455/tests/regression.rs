use task_455::*;

#[test]
fn transfer_case() {
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            first_nonempty_field_hash(&mut data);
            data
        },
        ""
    );
}
