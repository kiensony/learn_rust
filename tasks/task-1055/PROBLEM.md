# task-1055 — Use an associated type · closing marks removed (:)

Treat ':' as the field separator. Select the closing marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn closing_marks_removed_colon(packet: &impl Packet) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `closing_marks_removed_colon(&"hi:kit")` gives `6`.
- `closing_marks_removed_colon(&String::from(":é:a"))` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1055/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1055`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
