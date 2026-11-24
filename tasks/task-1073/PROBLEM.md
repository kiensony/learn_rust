# task-1073 — Use an associated type · leading marks removed (|)

Read only the last line, ignoring an optional final newline. Empty input gives an empty line. Treat '|' as the field separator. Select the leading marks removed. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn leading_marks_removed_pipe(packet: &impl Packet) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `leading_marks_removed_pipe(&"first\n123")` gives `3`.
- `leading_marks_removed_pipe(&String::from("header\n猫|123"))` gives `7`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1073/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1073`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
