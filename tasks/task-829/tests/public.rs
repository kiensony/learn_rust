use task_829::*;

#[test]
fn public_examples() {
    assert_eq!(combine(Ok(3), Ok(2)), Ok(5));
    assert_eq!(
        combine(Err(Field::Left), Err(Field::Right)),
        Err(Field::Left)
    );
}
