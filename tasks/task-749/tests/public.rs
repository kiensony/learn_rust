use task_749::*;

#[test]
fn public_examples() {
    assert_eq!(
        remainder_after_first_mark_hash(Some("outside[red#12]ignored")),
        Some(12)
    );
    assert_eq!(remainder_after_first_mark_hash(Some("[猫###a]")), None);
}
