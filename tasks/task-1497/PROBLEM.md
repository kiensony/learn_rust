# task-1497 — Client error fixture · run ledger

Build the public decoder used by an external integration test. Accept ASCII whitespace-separated three-digit statuses from 100 through 599. Ignore malformed tokens. Observe all 4xx codes. Add a separate tests/consumer.rs that calls the crate as a downstream consumer and covers a malformed token among valid tokens.

Return one (value, length) pair per maximal run of adjacent equal observations. Keep nonadjacent runs separate and preserve encounter order. Empty input yields an empty ledger.

Use an integration test outside src and the crate's public API. Add an independent regression that exercises both the source operation and this assessment's output contract. Explain why your case distinguishes two plausible implementations.

```rust
pub fn client_error_fixture_run_ledger(input: &str) -> Vec<(u32, usize)>
```

Work from the contract. Predict one boundary case and add it to `tests/regression.rs`. Explain how its failure would identify a specific mistake.

## Public examples

- `client_error_fixture_run_ledger("200 503 404 200")` gives `vec![(404u32, 1usize)]`.
- `client_error_fixture_run_ledger("")` gives `vec![]`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1497/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1497`.

[Module reading](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
