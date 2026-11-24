use task_1233::*;

#[test]
fn public_examples() {
    assert_eq!(seek_offsets_bounds(&[]), None);
    assert_eq!(seek_offsets_bounds(&[4]), None);
}
