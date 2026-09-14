# task-002 — Classify

```rust
pub fn classify(n: i64) -> &'static str
```

- Multiple of 3 and 5 → `"fizzbuzz"`
- Multiple of 3 → `"fizz"`
- Multiple of 5 → `"buzz"`
- Else → `"plain"`
- `0` is a multiple of both → `"fizzbuzz"`
- Negatives use the same rules (`-15` → `"fizzbuzz"`).

No `unsafe`. Std only.
