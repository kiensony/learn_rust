use task_427::*;

#[test]
fn public_examples() {
    assert_eq!(
        {
            let mut data = String::from("path/1/12");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "1"
    );
    assert_eq!(
        {
            let mut data = String::from("path/123");
            shortest_field_with_first_tie_slash(&mut data);
            data
        },
        "123"
    );
}
