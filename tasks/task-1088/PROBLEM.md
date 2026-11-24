# task-1088 — Use an associated type · first nonempty field (=)

Discard the comment starting at the first semicolon before examining fields. Treat '=' as the field separator. Select the first nonempty field. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Use the Packet trait’s associated Payload and return the selected byte length. Support every payload type implementing AsRef<str>.

```rust
pub fn first_nonempty_field_equals(packet: &impl Packet) -> usize
```

Implement the public function in `src/lib.rs`.

## Public examples

- `first_nonempty_field_equals(&"72;12")` gives `2`.
- `first_nonempty_field_equals(&String::from("猫=12;note"))` gives `3`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1088/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1088`.

[Module reading](https://doc.rust-lang.org/reference/items/traits.html)
