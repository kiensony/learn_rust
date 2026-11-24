# task-2998 — Transfer assessment · migrate a public wire version

Read exactly three bytes: a version tag and a two-byte integer. Version 1 is little-endian. Add support for version 2, which is big-endian. Return Err("length") for an incorrect length before examining the tag, and Err("version") for all unsupported versions. Diagnose the starter with an asymmetric byte pattern. Add an old-version regression plus a new-version case, and explain the compatibility risk if a decoder guessed the version from its payload.

```rust
pub fn read_versioned(input: &[u8]) -> Result<u16, &'static str>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `read_versioned(&[1,1,2])` gives `Ok(513)`.
- `read_versioned(&[2,1,2])` gives `Ok(258)`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-2998/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-2998`.

[Module reading](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html)
