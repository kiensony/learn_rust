# task-859 — Expose a module function · prefix before final mark (:)

Treat ':' as the field separator. Select the prefix before final mark. Measure lengths in UTF-8 bytes. For a missing separator, the remainder after the first mark and the prefix before the final mark are empty. Return the field’s byte length including the header size in wire::HEADER_BYTES. Keep wire private. Add a rustdoc example exercising the public function.

```rust
pub fn prefix_before_final_mark_colon(text: &str) -> usize
```

Replace the starter’s incomplete special case with a private helper implementing the general rule. Keep the public API. Explain which cases can now share the same code.

## Public examples

- `prefix_before_final_mark_colon("hi:kit")` gives `4`.
- `prefix_before_final_mark_colon(":é:a")` gives `5`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-859/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-859`.

[Module reading](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
