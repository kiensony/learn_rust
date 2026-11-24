# task-061 — Name the number on a locker

A variable is a name for a value. Here locker holds a whole number. Use the ticket number supplied by the caller as its value, then return it from the function. The last line reads the value of locker. In this exercise u8 can hold any whole number from 0 through 255.

```rust
pub fn locker_number(ticket: u8) -> u8
```

Implement the public function in `src/lib.rs`.

Read the signature from left to right. Parentheses contain the inputs. The type after `->` tells you what the caller receives. Predict the first example before editing.


## Public examples

- `locker_number(17)` gives `17`.
- `locker_number(42)` gives `42`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-061/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-061`.

[Module reading](https://doc.rust-lang.org/book/ch03-02-data-types.html)
