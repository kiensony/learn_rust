use task_518::*;

#[test]
fn public_examples() {
    assert_eq!(scalar_at("café", 3), Some('é'));
    assert_eq!(scalar_at("灯火", 1), Some('火'));
}
