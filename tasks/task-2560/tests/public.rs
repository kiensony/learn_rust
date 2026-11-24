use task_2560::acknowledge;

#[test]
fn acknowledge_one_sample() {
    assert_eq!(acknowledge(Box::new([9])), 1);
}

#[test]
fn acknowledge_a_batch() {
    assert_eq!(acknowledge(Box::new([9, 4, 2])), 3);
}
