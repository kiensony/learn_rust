use task_009::{Stack, Summary, TRACK_PROOF};

struct Note(&'static str);
impl Summary for Note {
    fn summarize(&self) -> String {
        format!("note:{}", self.0)
    }
}

#[test]
fn proof() {
    assert_eq!(TRACK_PROOF, 0x52535400 | 9);
}

#[test]
fn summarize_does_not_pop() {
    let mut s = Stack::new();
    s.push(Note("a"));
    s.push(Note("b"));
    assert_eq!(s.summarize_top().as_deref(), Some("note:b"));
    assert_eq!(s.len(), 2);
    assert_eq!(s.pop().map(|n| n.0), Some("b"));
}

#[test]
fn empty_summary() {
    let s: Stack<Note> = Stack::new();
    assert_eq!(s.summarize_top(), None);
}
