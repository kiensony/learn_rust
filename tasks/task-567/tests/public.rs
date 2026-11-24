use task_567::*;

#[test]
fn public_examples() {
    assert_eq!(left_caption(" north ", "south"), "north");
    assert_eq!(left_caption("w", "e"), "w");
}
