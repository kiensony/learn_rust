# task-006 — Rect

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub fn new(w: u32, h: u32) -> Option<Self>;
    pub fn area(&self) -> u32;
    pub fn can_hold(&self, other: &Rect) -> bool;
}
```

- `new` returns `None` if `w == 0` or `h == 0`.
- `area` is `w * h` (tests stay in range that does not overflow `u32`).
- `can_hold` is true when `self.w >= other.w && self.h >= other.h`.
- Derive the traits listed. Fields public.

No `unsafe`.
