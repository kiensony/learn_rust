use task_456::*;

#[test]
fn transfer_case() {
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            longest_field_with_last_tie_hash(&mut data);
            data
        },
        ""
    );
}
