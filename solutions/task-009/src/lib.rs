// rust-track: handwritten task-009
pub const TRACK_PROOF: u32 = 0x52535400 | 9;

pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
impl<T: Summary> Stack<T> {
    pub fn summarize_top(&self) -> Option<String> {
        self.items.last().map(Summary::summarize)
    }
}
