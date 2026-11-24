use task_457::*;

#[test]
fn transfer_case() {
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            shortest_field_with_first_tie_hash(&mut data);
            data
        },
        ""
    );
}
