# task-062 — Read the open sign

A boolean has two possible values, true and false. Bind the supplied is_open value to a local name and return that name. The sign should follow the input whenever it changes.

```rust
pub fn open_sign(is_open: bool) -> bool
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `open_sign(true)` gives `true`.
- `open_sign(false)` gives `false`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-062/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-062`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
