# task-2679 — Lending checksum · longest run

The starter specializes its intermediate representation in a way that loses information. Generalize it to support every input below. Move the observation step into a private helper and explain the ownership of its result during review.

Process consecutive borrowed chunks of up to 4 bytes. For each nonempty chunk observe the bytewise XOR. Do not allocate a copy of the chunk. During review connect the lifetime of each view with the API that produced it.

Return the length of the longest adjacent run of equal observations. Empty input yields zero. Use a generic associated type tied to the receiver borrow. During review, point to the operation that enforces its contract.

```rust
pub fn lending_checksum_longest_run(input: &[u8]) -> u64
```

## Public examples

- `lending_checksum_longest_run(&[])` gives `0`.
- `lending_checksum_longest_run(&[4])` gives `1`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2679/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2679`.

[Module reading](https://doc.rust-lang.org/reference/items/associated-items.html)
