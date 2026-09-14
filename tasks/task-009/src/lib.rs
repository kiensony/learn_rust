pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Stack<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn push(&mut self, item: T) {
        let _ = item;
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    pub fn len(&self) -> usize {
        0
    }
}

impl<T: Summary> Stack<T> {
    pub fn summarize_top(&self) -> Option<String> {
        None
    }
}
