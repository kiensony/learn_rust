use task_006::{Rect, TRACK_PROOF};

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 6);
}

#[test]
fn zero_h() {
    assert_eq!(Rect::new(4, 0), None);
}

#[test]
fn hold() {
    let a = Rect::new(5, 5).unwrap();
    let b = Rect::new(5, 5).unwrap();
    let c = Rect::new(6, 4).unwrap();
    assert!(a.can_hold(&b));
    assert!(!a.can_hold(&c));
}

#[test]
fn copy_eq() {
    let a = Rect::new(1, 2).unwrap();
    let b = a;
    assert_eq!(a, b);
}
