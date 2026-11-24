# task-1577 — A native archive that vanishes at link time

A build works through type checking and then fails while linking its executable. Run the link_probe integration test with --nocapture and locate the missing archive name in the linker's diagnostic. Its fixture builds a real native static archive with rustc. Implement the build tool's naming contract: replace every hyphen in a logical component name by an underscore and prefix it with track_. Identifiers supplied here contain lowercase ASCII letters, digits, hyphens and underscores. Cargo check alone cannot exercise the failing link step. Add a regression for a name with two hyphens.

```rust
pub fn native_archive(component: &str) -> String
```

Reproduce the failure before editing. Pick one input or compiler span, explain its cause, repair that case, and rerun both test suites.

## Public examples

- `native_archive("io")` gives `"track_io"`.
- `native_archive("metrics-v1")` gives `"track_metrics_v1"`.

## Run this exercise

From the course root, run `cargo test --manifest-path tasks/task-1577/Cargo.toml --test public`. After the examples pass, complete the submission step in REVIEW.md and run `cargo track test task-1577`.

[Module reading](https://doc.rust-lang.org/cargo/reference/features.html)
