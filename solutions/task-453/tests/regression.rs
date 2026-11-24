use task_453::*;

#[test]
fn transfer_case() {
    assert_eq!(
        {
            let mut data = String::from("ok#🐈");
            remainder_after_first_mark_hash(&mut data);
            data
        },
        ""
    );
}
