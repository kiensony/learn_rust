use task_009::Stack;

#[test]
fn push_pop() {
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    assert_eq!(s.len(), 2);
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.pop(), None);
}
