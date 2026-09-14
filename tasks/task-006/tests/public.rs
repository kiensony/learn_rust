use task_006::Rect;

#[test]
fn new_and_area() {
    let r = Rect::new(2, 3).unwrap();
    assert_eq!(r.area(), 6);
}

#[test]
fn reject_zero() {
    assert_eq!(Rect::new(0, 3), None);
}
