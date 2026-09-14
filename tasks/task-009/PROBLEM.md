# task-009 — Stack + Summarize

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Stack<T> {
    // private
}

impl<T> Stack<T> {
    pub fn new() -> Self;
    pub fn push(&mut self, item: T);
    pub fn pop(&mut self) -> Option<T>;
    pub fn len(&self) -> usize;
}

impl<T: Summary> Stack<T> {
    pub fn summarize_top(&self) -> Option<String>;
}
```

- Last-in first-out.
- `summarize_top` maps the top item through `Summary` without popping.
- No `unsafe`. Keep fields private.
