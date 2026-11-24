# task-071 — Keep one character in a ticket

Return a pair containing the original character and whether it is the lone star character *. A char is one Unicode scalar value. Rust writes character literals inside single quotes. The == operator tests equality and produces a boolean.

```rust
pub fn ticket_mark(mark: char) -> (char, bool)
```

Implement the public function in `src/lib.rs`.

## Public examples

- `ticket_mark('A')` gives `('A',false)`.
- `ticket_mark('*')` gives `('*',true)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-071/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-071`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
