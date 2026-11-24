use task_1223::*;

#[test]
fn public_examples() {
    assert_eq!(delimited_fields_bounds(&[]), None);
    assert_eq!(delimited_fields_bounds(&[4]), Some((1u32, 1u32)));
}
